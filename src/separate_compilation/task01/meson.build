srcs_tasks = [
  'AssignmentCoordinatorTaskService.cpp',
]

incs_tasks = [
  include_directories(
    '../../../src/Includes',
    '../../../src/Services',
    '../../../src/Communications',
    '../../../src/Utilities',
    '../../../src/Plans',
    '../../../src/DPSS',
    '../../../src/VisilibityLib',
    '../../../src/LMCP',
  ),
  include_directories(
    '../../../src/Tasks',
  ),
]

if get_option('afrl_internal')
  incs_tasks += incs_tasks_internal
else
  srcs_tasks_internal = files()
endif

lib_task01 = static_library(
  'task01',
  srcs_tasks,
  srcs_tasks_internal,
  dependencies: [
    dep_boost,
    dep_cppzmq,
    dep_pugixml,
    dep_zeromq,
  ],
  cpp_args: cpp_args,
  include_directories: incs_tasks
)
