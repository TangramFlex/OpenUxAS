#!/usr/bin/env python3

import itertools
import subprocess

if __name__ == '__main__':
    whitelist = [
        'CreateDpss',
        'DestroyDpss',
        'SmoothPath',
        'SetObjective',
        'AddVehicles',
        'RemoveVehicles',
        'UpdateVehicleTelemetry',
        'SetOutputPath',
        'SetLostCommWaypointNumber',
        'UpdateDpss',
    ]
    whitelist_args = itertools.chain.from_iterable([['--whitelist-function', fn] for fn in whitelist])
    cmd = [
        'bindgen', '../Dpss.h',
        '-o', 'src/bindings.rs',
        '--opaque-type', 'std::.*',
        '--opaque-type', 'Dpss',
        '--ignore-methods',
        '--raw-line', '#![allow(non_upper_case_globals)]',
        '--raw-line', '#![allow(non_camel_case_types)]',
        '--raw-line', '#![allow(non_snake_case)]',
    ] + list(whitelist_args) + [
        '--',
        '-x', 'c++',
        '-I../../Includes',
        '-I../../Utilities',
    ]
    subprocess.run(cmd)
