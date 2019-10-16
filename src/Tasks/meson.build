### Temporary edits:
# AssignmentCoordinatorTaskService.cpp -- disaggregated to task01
## ... -- Not infrastructure; additional disaggregation candidates

srcs_tasks = [
	'RendezvousTask.cpp',  # retain per 00_ServiceList.h instruction
  '00_TaskTemplate.cpp',
  'AngledAreaSearchTaskService.cpp',
#  'AssignmentCoordinatorTaskService.cpp',
  'BlockadeTaskService.cpp',
  'CmasiAreaSearchTaskService.cpp',
  'CmasiLineSearchTaskService.cpp',
  'CmasiPointSearchTaskService.cpp',
  'CommRelayTaskService.cpp',
  'CordonTaskService.cpp',
  'EscortTaskService.cpp',
  'ImpactLineSearchTaskService.cpp',
  'MustFlyTaskService.cpp',
  'LoiterTaskService.cpp',
  'ImpactPointSearchTaskService.cpp',
  'MultiVehicleWatchTaskService.cpp',
  'OverwatchTaskService.cpp',
  'PatternSearchTaskService.cpp',
  'TaskManagerService.cpp',
  'TaskServiceBase.cpp',
  'TaskTrackerService.cpp',
  'DynamicTaskServiceBase.cpp'
]

incs_tasks = [
  include_directories(
    '../../src/Includes',
    '../../src/Services',
    '../../src/Communications',
    '../../src/Utilities',
    '../../src/Plans',
    '../../src/DPSS',
    '../../src/VisilibityLib',
  ),
  incs_lmcp,
]

if get_option('afrl_internal')
  incs_tasks += incs_tasks_internal
else
  srcs_tasks_internal = files()
endif

lib_tasks = static_library(
  'tasks',
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
