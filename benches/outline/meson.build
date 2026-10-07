project('benchmarks', 'cpp', default_options : ['cpp_std=c++17'])

cmake = import('cmake')
options = cmake.subproject_options()
options.add_cmake_defines({'BENCHMARK_ENABLE_TESTING': false})
google_benchmark = cmake.subproject('google-benchmark', options: options)
google_benchmark_dep = google_benchmark.dependency('benchmark')

stb_truetype = subproject('stb_truetype')
stb_truetype_dep = stb_truetype.get_variable('stb_truetype_dep')

freetype_dep = dependency('freetype2', version : '>=2.8')

# maybe there is a better way
ttf_parser_dep = meson.get_compiler('cpp').find_library('ttfparser',
    dirs : [meson.current_source_dir() + '/../../c-api/target/release'])

executable('outline', 'outline.cc',
    dependencies : [google_benchmark_dep, freetype_dep, stb_truetype_dep, ttf_parser_dep],
    include_directories : ['../../c-api'],
    override_options : ['cpp_std=c++17'])
