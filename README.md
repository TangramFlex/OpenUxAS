# License

*OpenUxAS* is developed by the Air Force Research Laboratory, Aerospace System Directorate, Power and Control Division.
The LMCP specification and all source code for *OpenUxAS* is publicaly released under the Air Force Open Source Agreement
Version 1.0. See LICENSE.md for complete details. The Air Force Open Source Agreement closely follows the NASA Open Source
Agreement Verion 1.3.

**NOTE: The terms of the license include registering use of the software by emailing <a href="mailto:afrl.rq.opensource@us.af.mil?subject=OpenUxAS Registration&body=Please register me for use of OpenUxAS. Name: ____________">afrl.rq.opensource@us.af.mil</a>.**

# Introduction

UxAS consists of a collection of modular services that interact via a common message passing architecture. Similar in design to Robot Operating System (ROS), each service subscribes to messages in the system and responds to queries. UxAS uses the open-source library ZeroMQ to connect all services to each other. The content of each message conforms to the Light-weight Message Control Protocol (LMCP) format. Software classes providing LMCP message creation, access, and serialization/deserialization are automatically generated from simple XML description documents (see the *LmcpGen* project). These same XML descriptions detail the exact data fields, units, and default values for each message. Since all UxAS services communicate with LMCP formatted messages, a developer can quickly determine the input/output data for each service. In a very real sense, the message traffic in the system exposes the interaction of the services that are required to achieve autonomous behavior.

Consider a simple example: the automated construction of the flight pattern to conduct surveillance of geometric lines (e.g. perimeters, roads, coasts). A “line search task” message describes the line to be imaged and the desired camera angle. Using this input description, a line search service calculates the appropriate waypoints to achieve the proper view angle. When the UAV arrives at the first waypoint corresponding to the line search task, the line search service continuously updates the desired camera pointing location to smoothly step the camera along the intended route.

In addition to surveillance pattern automation, UxAS contains services that automate route planning, coordinate behavior among multiple vehicles, connect with external software, validate mission requests, log and diagram message traffic, and optimize task ordering. In all, UxAS has approximately 30 services.

A core functionality provided by UxAS is the mechanism to calculate near-optimal task allocation across teams of unmanned vehicles. With a collection of tasks that require servicing and a pool of vehicles available to service those tasks, UxAS is able to determine which vehicle should do which task in the proper order. This task assignment pipeline is carried out by a series of services working together in a complex sequence.

# Supported Operating Systems

For an Ubuntu 16.04, Fedora 28 or Mac OS X system with prerequisites installed, UxAS should build from source without issue. Support for Windows is available on Windows 7 and 10 using Visual Studio.

*NOTE: As this project is in transition, not all build methods are up-to-date. Refer to `BUILDERS_STATUS` for the last-known state of each method.*

# Configure Host System and Perform Initial UxAS Build

For Linux and Mac systems, the command `bash install_prerequisites.sh` automates the installation of all the necessary tools for compilation of *OpenUxAS* and performs the initial build.

*NOTE: On Mac OS X, [XCode](https://developer.apple.com/xcode/) must first be installed before running the install script.

Subsequent builds are performed using `ninja`.


## Building at the Command Line
1. From the *OpenUxAS* local repository (i.e. `cd OpenUxAS`)
1. The `install_prerequisites.sh` script creates a release build in the
   `build` directory and a debug build in the `build_debug` directory.
   Either build directory may be specified in the following commands.
1. Build UxAS: in terminal
   * `ninja -C build all`
   * This step is the only step necessary in day-to-day development work. It's
the Meson equivalent of `make all`.
   * To clean the build, add the `clean` target at the end of your ninja
command: `ninja -C build clean`
1. Run UxAS tests: in terminal
   * `ninja -C build test`
   * Confirm all tests passed

### Compiling using NetBeans (Debug Mode)

1. Install [NetBeans and Oracle Java JDK](http://www.oracle.com/technetwork/java/javase/downloads/jdk-netbeans-jsp-142931.html)
1. Enable C/C++ plug-in in NetBeans
   - Choose Tools->Plugins from the top menu
   - In the `Available Plugins` tab, search for `C++`
   - Select `C/C++` and click `Install`
1. Select File->New Project
1. Choose `C/C++ Project with Existing Sources` and click `Next`
1. Specify the `OpenUxAS` folder
1. Select the `Custom` option under `Select Configuration Mode` and click `Next`
1. No changes under `Pre-Build Action`, click `Next`
1. Set the `Clean Command` to `ninja -C build_debug clean`
1. Set the `Build Command` to `ninja -C build_debug uxas` and click `Next`
1. No changes under `Source Files`, click `Next`
1. No changes under `Code Assistance Configuration`, click `Next`
1. Change `Project Name` to `UxAS` and click `Finish`

For Linux systems, Netbeans will automatically use the `gdb` debugger. On Mac OS X,
`gdb` must be installed and signed (see [Neil Traft's guide](http://ntraft.com/installing-gdb-on-os-x-mavericks/)).

# Running the Examples

These examples require a windowing environment for the *OpenAMASE* application.

1. Run examples
   * Example 2: Follow README.md in `examples/02_Example_WaterwaySearch`
   * Example 3: Follow README.md in `examples/03_Example_DistributedCooperation`

# Building and Viewing the Documentation

For Linux and Mac systems, the command `bash build_document.sh` installs required tools and builds *OpenUxAS* documentation.

The following files which appear at the top of the *OpenUxAS* directory tree depend upon the documentation having been built:

1. UxAS_UserManual.pdf
1. browse_doxygen.sh
1. browse_lmcp.sh

# About External Dependencies

In porting the UxAS build system to Meson/Ninja, we've taken advantage of
`wrap` facility to import and build 3rd-party libraries. The advantage
of this approach is that the main UxAS repo no longer needs to contain
these libraries.

There are some rough edges. The `wrap` facility (as of April 2017) was
designed to store the necessary metadata on a server operated by the
Meson/Ninja maintainers. There's a very short list of wrapped projects
available from this server. Worse, the `wrap` facility is not properly
designed for project-local use: "patches" (often, only the necessary
`meson.build` file) are downloaded by the `wrap` facility, which offers
no provision for relative URLs.

Furthermore, the `patch` file must be in an archive format. This means
that the wrapped project's `meson.build` file must be tarred (actually,
the `wrap` facility will handle other archive formats) for reference
from the project's wrap file, and the wrap file must contain a valid
SHA256 hash of the patch archive file.

Clearly, this will complicate maintenance. On the plus side, once an
external project is properly wrapped, it shouldn't require further work
unless you require a different version of the project.

We've taken the approach of stashing valid `meson.build` files in the
`3rd/wrap_patches` directory. This is the place to store other patched
files (if any) needed for the build of the external project. Note that
"patch" does not refer to a context or unified diff, but rather to an
archive containing new and changed files that overwrite the unzipped
sources. The `wrap` facility is not able to patch using diff files.

## About Boost

Boost is handled slightly differently from the other external
dependencies, in that the build system attempts to use a
system-provided version of Boost before falling back on the `wrap`
facility as a last resort.

Boost uses a bespoke configuration and build system
that is very difficult to replicate with a Meson-based `wrap` build,
and so Meson itself handles Boost differently from other
`pkg-config`-provided system dependencies.

### System-provided Boost

We *strongly* recommend using a system-provided Boost from `brew`,
`apt-get`, etc. If you have a system-provided boost, but during
Meson's configuration phase, you see something like the following, try
setting your `BOOST_ROOT` environment variable to the prefix of your
system-installed packages (most likely `/usr/local` for MacOS with
Homebrew):

```
Dependency Boost (filesystem, regex, system) found: NO
```

If you have a system-provided Boost but this message still does not go
away, [open an issue](https://github.com/afrl-rq/OpenUxAS/issues/new)
with details of your system configuration.

### Boost via Meson wrap

If no system-provided Boost is available, Meson will fall back to
using the `wrap` we maintain alongside the other external
dependencies. This will probably work on 64-bit Linux systems, but
unexpected trouble may arise on other platforms.

### Removing External Dependencies

If you ever feel the need to refresh external dependencies, you'll need
to remove both the downloaded files and the expanded directories:

`./rm-external`

This script depends upon the presence of the patch tarballs installed
in the `/3rd` directory by `./prepare`.



### Caveats

- The Visual Studio backend for Meson mostly works, but will fail when regenerating build files. If you modify one of the `meson.build` files, delete the `build` directory and run `meson.py build --backend=vs` again. The steps following the `meson.build` command must also be performed.
- The UxAS test suite uses some hardcoded POSIX-style paths, and so does not currently work on Windows.

---
---

# CAUTION

This section contains materials and descriptions which may be outdated or incorrect. This material is retained for reference and shall be removed as the various workflow and platform scripts are brought up-to-date.

## Prep and Build on Windows

1. Install [Visual Studio 2017 Community Edition](https://www.visualstudio.com/downloads/)
   * Ensure C++ selected in `Workloads` tab
   * Ensure `Git for Windows` is selected in `Individual components` tab
1. Install [Git](https://git-scm.com/download/win) with Bash shell
1. Install [Python 3](https://www.python.org/ftp/python/3.7.0/python-3.7.0.exe)
   * Make sure to check `Add Python 3.7 to PATH`
   * Choose standard install (`Install Now`, requires admin)
   * Verify installation by: `python --version` in `cmd` prompt
   * Verify *pip* is also installed: `pip --version` in `cmd` prompt
   * If unable to get python on path, follow [this answer](https://stackoverflow.com/questions/23400030/windows-7-add-path) using location `C:\Users\[user]\AppData\Local\Programs\Python\Python37-32\`
1. Install *meson* (due to Boost linking difficulty, a patched version of meson is required)
   * In Git Bash shell: `git -c http.sslVerify=false clone https://github.com/derekkingston/meson.git`
   * Install *meson* in Git Bash shell: `cd meson; python setup.py install`
1. Install [Boost 1.67](https://dl.bintray.com/boostorg/release/1.67.0/binaries/boost_1_67_0-msvc-14.1-32.exe)
   * Note: the above link is for VS2017 pre-compiled libraries. To compile from source, you must install at the location: `C:\local\boost_1_67_0`
1. Pull UxAS repositories (from Git Bash shell)
   * `git -c http.sslVerify=false clone https://github.com/afrl-rq/OpenUxAS.git`
   * `git -c http.sslVerify=false clone https://github.com/afrl-rq/LmcpGen.git`
   * `git -c https://github.com/afrl-rq/OpenAMASE.git`
1. (**optional**) Build OpenAMASE or [download](https://github.com/afrl-rq/OpenAMASE/releases/download/v1.3.1/OpenAMASE.jar) and place in the `OpenAMASE\OpenAMASE\dist` directory
   * Load the OpenAMASE project in NetBeans and click `Build`
1. Auto-create the UxAS messaging library
   * Download released executable from [GitHub](https://github.com/afrl-rq/LmcpGen/releases/download/v1.7.1/LmcpGen.jar)
   * Place `LmcpGen.jar` in `LmcpGen/dist` folder
   * From the Git Bash shell in the root UxAS directory, run `bash RunLmcpGen.sh`
   * Note: For simplicity, make sure the LMCPGen, OpenUxAS, and OpenAMASE repositories follow the folder structure labeled in the [Build UxAS](#build-uxas) section.
1. Prepare build
   * Open VS command prompt (Tools -> Visual Studio Command Prompt)
   * Note: If the Visual Studio Command Prompt is absent from Visual Studio, it is also possible to perform the following actions by searching for the `Developer Command Prompt for VS 2017` application and switching the working directory to the root OpenUxAS directory
   * `python prepare`
   * `meson.py build --backend=vs`
   * A Visual Studio solution named `UxAS.sln` will be in the `build` folder
1. Build project with Visual Studio
   * Open project file `UxAS.sln` in the `OpenUxAS/build` directory
   * (**optional**) Remove `REGEN`, `RUN_INSTALL`, and `RUN_TESTS` projects from the solution
   * In the `Solution Explorer`, right-click the `uxas` project, and select `Build` from the context menu
