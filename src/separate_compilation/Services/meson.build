srcs_services = [
  '00_ServiceTemplate.cpp',
  '01_HelloWorld.cpp',
  'ServiceBase.cpp',
  'ServiceManager.cpp',
]

incs_services = [
  include_directories(
    '../../../src/Utilities',
    '../../../src/Communications',
    '../../../src/Includes',
    '../../../src/separate_compilation/Services',
    '../../../src/VisilibityLib',
    '../../../src/Plans',
    '../../../src/DPSS',
    '../../../src/Tasks',
  ),
  incs_lmcp,
]

deps_services = [
  dep_boost,
  dep_cppzmq,
  dep_pugixml,
  dep_sqlite3,
  dep_sqlitecpp,
  dep_zeromq,
]

if get_option('afrl_internal')
  deps_services += deps_services_internal
  incs_services += incs_services_internal
else
  srcs_services_internal = files()
endif

lib_services_task01 = static_library(
  'services',
  srcs_services,
  srcs_services_internal,
  dependencies: deps_services,
  cpp_args: cpp_args,
  include_directories: incs_services,
)
