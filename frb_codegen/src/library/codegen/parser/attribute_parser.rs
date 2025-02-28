use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
use crate::codegen::ir::annotation::IrDartAnnotation;
use crate::codegen::ir::default::IrDefaultValue;
use crate::codegen::ir::import::IrDartImport;
use crate::codegen::ir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::if_then_some;
use anyhow::Context;
use itertools::Itertools;
use serde::{Serialize, Serializer};
use syn::parse::{Lookahead1, Parse, ParseStream, Peek};
use syn::punctuated::Punctuated;
use syn::*;

const METADATA_IDENT: &str = "frb";

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct FrbAttributes(Vec<FrbAttribute>);

impl FrbAttributes {
    pub(crate) fn parse(attrs: &[Attribute]) -> anyhow::Result<Self> {
        Ok(Self(
            attrs
                .iter()
                .filter(|attr| {
                    attr.path().segments.last().unwrap().ident == METADATA_IDENT
                        // exclude the `#[frb]` case
                        && !matches!(attr.meta, Meta::Path(_))
                })
                .map(|attr| {
                    attr.parse_args::<FrbAttributesInner>()
                        .with_context(|| format!("attr={:?}", quote::quote!(#attr).to_string()))
                })
                .collect::<anyhow::Result<Vec<_>>>()?
                .into_iter()
                .flat_map(|x| x.0)
                .collect(),
        ))
    }

    pub(crate) fn default_value(&self) -> Option<IrDefaultValue> {
        let candidates = self
            .0
            .iter()
            .filter_map(
                |item| if_then_some!(let FrbAttribute::Default(default) = item, default.clone()),
            )
            .collect_vec();
        if candidates.len() > 1 {
            // We do not care about details of this warning message
            // frb-coverage:ignore-start
            log::warn!("Only one `default = ..` attribute is expected; taking the last one");
            // frb-coverage:ignore-end
        }
        candidates.last().map(|item| item.to_ir_default_value())
    }

    pub(crate) fn non_final(&self) -> bool {
        self.any_eq(&FrbAttribute::NonFinal)
    }

    pub(crate) fn sync(&self) -> bool {
        self.any_eq(&FrbAttribute::Sync)
    }

    pub(crate) fn getter(&self) -> bool {
        self.any_eq(&FrbAttribute::Getter)
    }

    pub(crate) fn init(&self) -> bool {
        self.any_eq(&FrbAttribute::Init)
    }

    pub(crate) fn ignore(&self) -> bool {
        self.any_eq(&FrbAttribute::Ignore)
    }

    pub(crate) fn opaque(&self) -> Option<bool> {
        if self.any_eq(&FrbAttribute::Opaque) {
            Some(true)
        } else if self.any_eq(&FrbAttribute::NonOpaque) {
            Some(false)
        } else {
            None
        }
    }

    pub(crate) fn generate_hash(&self) -> bool {
        !self.any_eq(&FrbAttribute::NonHash)
    }

    pub(crate) fn generate_eq(&self) -> bool {
        !self.any_eq(&FrbAttribute::NonEq)
    }

    pub(crate) fn rust_opaque_codec(&self) -> Option<RustOpaqueCodecMode> {
        if self.any_eq(&FrbAttribute::RustOpaqueCodecMoi) {
            Some(RustOpaqueCodecMode::Moi)
        } else {
            None
        }
    }

    pub(crate) fn codec_mode_pack(&self) -> Option<CodecModePack> {
        if self.any_eq(&FrbAttribute::Serialize) {
            Some(CodecModePack {
                dart2rust: CodecMode::Sse,
                rust2dart: CodecMode::Sse,
            })
        } else if self.any_eq(&FrbAttribute::SemiSerialize) {
            Some(CodecModePack {
                dart2rust: CodecMode::Cst,
                rust2dart: CodecMode::Sse,
            })
        } else {
            None
        }
    }

    fn any_eq(&self, target: &FrbAttribute) -> bool {
        self.0.iter().any(|item| item == target)
    }

    pub(crate) fn mirror(&self) -> Vec<Path> {
        self.0
            .iter()
            .filter_map(
                |item| if_then_some!(let FrbAttribute::Mirror(mirror) = item, mirror.0.clone()),
            )
            .flatten()
            .collect()
    }

    pub(crate) fn dart_metadata(&self) -> Vec<IrDartAnnotation> {
        self.0
            .iter()
            .filter_map(
                |item| if_then_some!(let FrbAttribute::Metadata(metadata) = item, metadata.value.0.clone()),
            )
            .flatten()
            .collect()
    }

    pub(crate) fn dart_code(&self) -> String {
        self.0
            .iter()
            .filter_map(
                |item| if_then_some!(let FrbAttribute::DartCode(inner) = item, inner.0.clone()),
            )
            .join("\n\n")
    }
}

mod frb_keyword {
    syn::custom_keyword!(mirror);
    syn::custom_keyword!(non_final);
    syn::custom_keyword!(sync);
    syn::custom_keyword!(getter);
    syn::custom_keyword!(init);
    syn::custom_keyword!(ignore);
    syn::custom_keyword!(opaque);
    syn::custom_keyword!(non_opaque);
    syn::custom_keyword!(non_hash);
    syn::custom_keyword!(non_eq);
    syn::custom_keyword!(rust_opaque_codec_moi);
    syn::custom_keyword!(serialize);
    syn::custom_keyword!(semi_serialize);
    syn::custom_keyword!(dart_metadata);
    syn::custom_keyword!(import);
    syn::custom_keyword!(default);
    syn::custom_keyword!(dart_code);
}

struct FrbAttributesInner(Vec<FrbAttribute>);

impl Parse for FrbAttributesInner {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(
            Punctuated::<FrbAttribute, Token![,]>::parse_terminated(input)?
                .into_iter()
                .collect(),
        ))
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum FrbAttribute {
    Mirror(FrbAttributeMirror),
    NonFinal,
    Sync,
    Getter,
    Init,
    Ignore,
    Opaque,
    NonOpaque,
    NonHash,
    NonEq,
    RustOpaqueCodecMoi,
    Serialize,
    // NOTE: Undocumented, since this name may be suboptimal and is subject to change
    SemiSerialize,
    Metadata(NamedOption<frb_keyword::dart_metadata, FrbAttributeDartMetadata>),
    Default(FrbAttributeDefaultValue),
    DartCode(FrbAttributeDartCode),
}

impl Parse for FrbAttribute {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        use frb_keyword::*;
        use FrbAttribute::*;

        let lookahead = input.lookahead1();

        let keyword_output = parse_keyword::<non_final, _>(input, &lookahead, non_final, NonFinal)
            .or_else(|| parse_keyword::<sync, _>(input, &lookahead, sync, Sync))
            .or_else(|| parse_keyword::<getter, _>(input, &lookahead, getter, Getter))
            .or_else(|| parse_keyword::<init, _>(input, &lookahead, init, Init))
            .or_else(|| parse_keyword::<ignore, _>(input, &lookahead, ignore, Ignore))
            .or_else(|| parse_keyword::<opaque, _>(input, &lookahead, opaque, Opaque))
            .or_else(|| parse_keyword::<non_opaque, _>(input, &lookahead, non_opaque, NonOpaque))
            .or_else(|| parse_keyword::<non_hash, _>(input, &lookahead, non_hash, NonHash))
            .or_else(|| parse_keyword::<non_eq, _>(input, &lookahead, non_eq, NonEq))
            .or_else(|| {
                parse_keyword::<rust_opaque_codec_moi, _>(
                    input,
                    &lookahead,
                    rust_opaque_codec_moi,
                    RustOpaqueCodecMoi,
                )
            })
            .or_else(|| parse_keyword::<serialize, _>(input, &lookahead, serialize, Serialize))
            .or_else(|| {
                parse_keyword::<semi_serialize, _>(input, &lookahead, semi_serialize, SemiSerialize)
            });
        if let Some(keyword_output) = keyword_output {
            return keyword_output;
        }

        Ok(if lookahead.peek(frb_keyword::mirror) {
            input.parse::<frb_keyword::mirror>()?;
            input.parse().map(FrbAttribute::Mirror)?
        } else if lookahead.peek(frb_keyword::dart_metadata) {
            input.parse().map(FrbAttribute::Metadata)?
        } else if lookahead.peek(default) {
            input.parse::<default>()?;
            input.parse::<Token![=]>()?;
            input.parse().map(FrbAttribute::Default)?
        } else if lookahead.peek(dart_code) {
            input.parse::<dart_code>()?;
            input.parse::<Token![=]>()?;
            input.parse().map(FrbAttribute::DartCode)?
        } else {
            return Err(lookahead.error());
        })
    }
}

fn parse_keyword<T: Parse, S: Peek>(
    input: ParseStream,
    lookahead: &Lookahead1,
    token: S,
    attribute: FrbAttribute,
) -> Option<Result<FrbAttribute>> {
    lookahead
        .peek(token)
        .then(|| input.parse::<T>().map(|_| attribute))
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NamedOption<K, V> {
    name: K,
    value: V,
}

impl<K: Parse + std::fmt::Debug, V: Parse> Parse for NamedOption<K, V> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name: K = input.parse()?;
        let _: Token![=] = input.parse()?;
        let value = input.parse()?;
        Ok(Self { name, value })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FrbAttributeMirror(Vec<Path>);

impl Parse for FrbAttributeMirror {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let paths = Punctuated::<Path, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Ok(Self(paths))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FrbAttributeDartMetadata(Vec<IrDartAnnotation>);

impl Parse for FrbAttributeDartMetadata {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let annotations = Punctuated::<IrDartAnnotation, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Ok(Self(annotations))
    }
}

// TODO unused, rm?
// #[derive(Clone, Debug)]
// struct DartImports(Vec<IrDartImport>);
//
// impl Parse for DartImports {
//     fn parse(input: ParseStream<'_>) -> Result<Self> {
//         let content;
//         parenthesized!(content in input);
//         let imports = Punctuated::<IrDartImport, Token![,]>::parse_terminated(&content)?
//             .into_iter()
//             .collect();
//         Ok(Self(imports))
//     }
// }

impl Parse for IrDartImport {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let uri: LitStr = input.parse()?;
        let alias: Option<String> = if input.peek(token::As) {
            let _ = input.parse::<token::As>()?;
            let alias: Ident = input.parse()?;
            Some(alias.to_string())
        } else {
            None
        };
        Ok(Self {
            uri: uri.value(),
            alias,
        })
    }
}

impl Parse for IrDartAnnotation {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let annotation: LitStr = input.parse()?;
        let library = if input.peek(frb_keyword::import) {
            let _ = input.parse::<frb_keyword::import>()?;
            let library: IrDartImport = input.parse()?;
            Some(library)
        } else {
            None
        };
        Ok(Self {
            content: annotation.value(),
            library,
        })
    }
}

#[derive(Clone, Serialize, Eq, PartialEq, Debug)]
enum FrbAttributeDefaultValue {
    #[serde(serialize_with = "serialize_litstr")]
    Str(syn::LitStr),
    #[serde(serialize_with = "serialize_litbool")]
    Bool(syn::LitBool),
    #[serde(serialize_with = "serialize_litint")]
    Int(syn::LitInt),
    #[serde(serialize_with = "serialize_litfloat")]
    Float(syn::LitFloat),
    #[serde(serialize_with = "serialize_punctuated")]
    Vec(Punctuated<FrbAttributeDefaultValue, Token![,]>),
}

impl Parse for FrbAttributeDefaultValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let lh = input.lookahead1();
        if lh.peek(token::Bracket) {
            let inner;
            bracketed!(inner in input);
            Punctuated::parse_terminated(&inner).map(Self::Vec)
        } else if lh.peek(syn::LitStr) {
            input.parse().map(Self::Str)
        } else if lh.peek(syn::LitBool) {
            input.parse().map(Self::Bool)
        } else if lh.peek(syn::LitFloat) {
            input.parse().map(Self::Float)
        } else if lh.peek(syn::LitInt) {
            input.parse().map(Self::Int)
        } else {
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            Err(lh.error())
            // frb-coverage:ignore-end
        }
    }
}

impl FrbAttributeDefaultValue {
    fn to_ir_default_value(&self) -> IrDefaultValue {
        match self {
            Self::Str(lit) => IrDefaultValue::String {
                content: lit.value(),
            },

            // other types
            Self::Bool(lit) => IrDefaultValue::Others {
                dart_literal: (if lit.value { "true" } else { "false" }).to_owned(),
            },
            Self::Int(lit) => IrDefaultValue::Others {
                dart_literal: lit.base10_digits().into(),
            },
            Self::Float(lit) => IrDefaultValue::Others {
                dart_literal: lit.base10_digits().into(),
            },
            Self::Vec(lit) => IrDefaultValue::Others {
                dart_literal: format!(
                    "const [{}]",
                    lit.iter()
                        .map(|item| item.to_ir_default_value().to_dart_literal().to_string())
                        .collect_vec()
                        .join(",")
                ),
            },
        }
    }
}

fn serialize_litstr<S: Serializer>(
    lit: &syn::LitStr,
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    lit.value().serialize(s)
}

fn serialize_litbool<S: Serializer>(
    lit: &syn::LitBool,
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    lit.value().serialize(s)
}

fn serialize_litint<S: Serializer>(
    lit: &syn::LitInt,
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    lit.base10_parse::<i64>().unwrap().serialize(s)
}

fn serialize_litfloat<S: Serializer>(
    lit: &syn::LitFloat,
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    lit.base10_parse::<f64>().unwrap().serialize(s)
}

fn serialize_punctuated<S: Serializer>(
    lit: &Punctuated<FrbAttributeDefaultValue, Token![,]>,
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    lit.into_iter().collect_vec().serialize(s)
}

#[derive(Clone, Serialize, Eq, PartialEq, Debug)]
struct FrbAttributeDartCode(String);

impl Parse for FrbAttributeDartCode {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<syn::LitStr>().map(|x| Self(x.value()))
    }
}

#[cfg(test)]
mod tests {
    use crate::codegen::ir::default::IrDefaultValue;
    use crate::codegen::parser::attribute_parser::{
        FrbAttribute, FrbAttributeDartCode, FrbAttributeDefaultValue, FrbAttributeMirror,
        FrbAttributes, NamedOption,
    };
    use crate::if_then_some;
    use quote::quote;
    use syn::ItemFn;

    #[test]
    fn test_error() -> anyhow::Result<()> {
        let result = parse("#[frb(what_is_this)]");
        assert!(result.err().is_some());
        Ok(())
    }

    #[test]
    fn test_double_colon() -> anyhow::Result<()> {
        let parsed = parse("#[flutter_rust_bridge::frb(sync)]")?;
        assert_eq!(parsed.0, vec![FrbAttribute::Sync]);
        Ok(())
    }

    #[test]
    fn test_multiple_via_comma() -> anyhow::Result<()> {
        let parsed = parse("#[frb(sync, non_final)]")?;
        assert_eq!(parsed.0, vec![FrbAttribute::Sync, FrbAttribute::NonFinal]);
        Ok(())
    }

    #[test]
    fn test_multiple_via_hash() -> anyhow::Result<()> {
        let parsed = parse("#[frb(sync)]\n#[frb(non_final)]")?;
        assert_eq!(parsed.0, vec![FrbAttribute::Sync, FrbAttribute::NonFinal]);
        Ok(())
    }

    #[test]
    fn test_empty() -> anyhow::Result<()> {
        let parsed = parse("#[frb]")?;
        assert_eq!(parsed.0, vec![]);
        Ok(())
    }

    #[test]
    fn test_empty_bracket() -> anyhow::Result<()> {
        let parsed = parse("#[frb()]")?;
        assert_eq!(parsed.0, vec![]);
        Ok(())
    }

    #[test]
    fn test_mirror() -> anyhow::Result<()> {
        let parsed = parse("#[frb(mirror(Apple, Orange))]")?;
        let paths = if_then_some!(let FrbAttribute::Mirror(FrbAttributeMirror(paths)) = &parsed.0[0], paths);
        let path = &paths.unwrap()[0];
        assert_eq!(quote!(#path).to_string(), "Apple");
        Ok(())
    }

    fn simple_keyword_tester(keyword_name: &str, attribute: FrbAttribute) {
        let parsed = parse(&format!("#[frb({keyword_name})]")).unwrap();
        assert_eq!(parsed, FrbAttributes(vec![attribute]));
    }

    #[test]
    fn test_non_final() {
        simple_keyword_tester("non_final", FrbAttribute::NonFinal);
    }

    #[test]
    fn test_sync() {
        simple_keyword_tester("sync", FrbAttribute::Sync);
    }

    #[test]
    fn test_getter() {
        simple_keyword_tester("getter", FrbAttribute::Getter);
    }

    #[test]
    fn test_init() {
        simple_keyword_tester("init", FrbAttribute::Init);
    }

    #[test]
    fn test_ignore() {
        simple_keyword_tester("ignore", FrbAttribute::Ignore);
    }

    #[test]
    fn test_opaque() {
        simple_keyword_tester("opaque", FrbAttribute::Opaque);
    }

    #[test]
    fn test_non_opaque() {
        simple_keyword_tester("non_opaque", FrbAttribute::NonOpaque);
    }

    #[test]
    fn test_non_hash() {
        simple_keyword_tester("non_hash", FrbAttribute::NonHash);
    }

    #[test]
    fn test_non_eq() {
        simple_keyword_tester("non_eq", FrbAttribute::NonEq);
    }

    #[test]
    fn test_rust_opaque_codec_moi() {
        simple_keyword_tester("rust_opaque_codec_moi", FrbAttribute::RustOpaqueCodecMoi);
    }

    #[test]
    fn test_dart_code() -> anyhow::Result<()> {
        let parsed = parse(r###"#[frb(dart_code="a\nb\nc")]"###)?;
        assert_eq!(
            parsed,
            FrbAttributes(vec![FrbAttribute::DartCode(FrbAttributeDartCode(
                "a\nb\nc".to_owned()
            ))])
        );
        Ok(())
    }

    #[test]
    fn test_metadata() -> anyhow::Result<()> {
        let parsed = parse(
            r#"#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]"#,
        )?;
        let value = if_then_some!(let FrbAttribute::Metadata(NamedOption { value, .. }) = &parsed.0[0], value).unwrap();
        assert_eq!(value.0[0].content, "freezed");
        assert_eq!(value.0[1].content, "immutable");
        Ok(())
    }

    #[test]
    fn test_default() -> anyhow::Result<()> {
        let parsed = parse(r#"#[frb(default = "Weekdays.Sunday")]"#)?;
        let value = if_then_some!(let FrbAttribute::Default(value) = &parsed.0[0], value).unwrap();
        assert!(matches!(value, FrbAttributeDefaultValue::Str(_)));
        Ok(())
    }

    // Mirror(FrbAttributeMirror),
    // NonFinal,
    // Sync,
    // Metadata(NamedOption<frb_keyword::dart_metadata, FrbAttributeDartMetadata>),
    // Default(FrbAttributeDefaultValue),

    fn parse(raw: &str) -> anyhow::Result<FrbAttributes> {
        let code = raw.to_owned() + " fn f() {}";
        let fn_ast: ItemFn = syn::parse_str(&code)?;
        FrbAttributes::parse(&fn_ast.attrs)
    }

    #[test]
    fn test_frb_attribute_default_value() -> anyhow::Result<()> {
        for (text, expect_ir_default_value) in vec![
            (
                "\"Hello\"",
                IrDefaultValue::String {
                    content: "Hello".to_string(),
                },
            ),
            (
                "true",
                IrDefaultValue::Others {
                    dart_literal: "true".to_string(),
                },
            ),
            (
                "100",
                IrDefaultValue::Others {
                    dart_literal: "100".to_string(),
                },
            ),
            (
                "1.5",
                IrDefaultValue::Others {
                    dart_literal: "1.5".to_string(),
                },
            ),
            (
                "[100,200]",
                IrDefaultValue::Others {
                    dart_literal: "const [100,200]".to_string(),
                },
            ),
        ] {
            let value: FrbAttributeDefaultValue = syn::parse_str(text)?;
            assert_eq!(value.to_ir_default_value(), expect_ir_default_value);
            assert!(!serde_json::to_string(&value)?.is_empty());
        }
        Ok(())
    }
}
