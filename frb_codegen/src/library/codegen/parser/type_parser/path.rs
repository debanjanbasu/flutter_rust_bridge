use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::type_parser::path_data::extract_path_data;
use crate::codegen::parser::type_parser::unencodable::splay_segments;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use anyhow::bail;
use quote::ToTokens;
use syn::{Path, QSelf, TypePath};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path(&mut self, type_path: &TypePath) -> anyhow::Result<IrType> {
        match &type_path {
            TypePath { qself: None, path } => self.parse_type_path_core(type_path, path),
            // This branch simply halts the generator with an error message, so we do not test it
            // frb-coverage:ignore-start
            TypePath {
                qself: Some(QSelf { ty, .. }),
                ..
            } => bail!(
                "qself \"<{}>\" in \"{}\", and all qself syntax, is unsupported",
                ty.to_token_stream(),
                type_path.to_token_stream()
            ),
            // frb-coverage:ignore-end
        }
    }

    fn parse_type_path_core(
        &mut self,
        type_path: &TypePath,
        path: &Path,
    ) -> anyhow::Result<IrType> {
        let segments = extract_path_data(path)?;
        let splayed_segments = splay_segments(&segments);

        if let Some(last_segment) = splayed_segments.last() {
            if let Some(ans) = self.parse_type_path_data_primitive(last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) =
                self.parse_type_path_data_concrete(last_segment, &splayed_segments)?
            {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_struct(last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_enum(last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_rust_opaque(last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_optional(type_path, last_segment)? {
                return Ok(ans);
            }

            // this bracket is weirdly not covered, while everything else is
            // frb-coverage:ignore-start
        }
        // frb-coverage:ignore-end

        self.parse_type_rust_auto_opaque(None, &syn::Type::Path(type_path.to_owned()))
    }
}
