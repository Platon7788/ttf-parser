project('ttf-parser', 'rust')

if not meson.get_compiler('rust').version().version_compare('>=1.99')
  error('Rust 1.99 or newer is required')
endif

# Cargo's feature defaults are not applied by Meson/direct rustc invocation.
add_project_arguments(['--edition=2024', '--cfg', 'feature="std"',
  '--cfg', 'feature="alloc"', '--cfg', 'feature="glyph-names"',
  '--cfg', 'feature="variable-fonts"', '--cfg', 'feature="opentype-layout"',
  '--cfg', 'feature="apple-layout"'], language: 'rust')

ttf_parser = static_library('ttf_parser_capi', 'c-api/lib.rs', rust_crate_type: 'staticlib',
  link_with: static_library('ttf_parser', 'src/lib.rs'),
)

ttf_parser_dep = declare_dependency(
  link_with: ttf_parser,
  include_directories: 'c-api/',
)
