# Licensor

A bare minimum cli tool to print out SPDX licenses to stdout.

### Usage

#### Just print to stdout

- `licensor Unlicense`
- `licensor 0BSD`

#### Create a LICENSE file

- `licensor Unlicense > LICENSE`

### Non-SPDX Licenses

We have support for non-SPDX licenses.
The current non-SPDX licenses we support are found in the [`licenses` directory](licenses).
[Submit a Pull Request](https://github.com/sean-clayton/licensor/pulls) if you wish to add more licenses.
Please keep the naming convention the same: `LicenseRef-IDENTIFIER`.

### Future Plans

- ~~**Default to stdout.** Make full use of the beauty of a cli. `licensor Unlicense > LICENSE`~~
- Add full CLI toolkit
  - `licensor --help` Self-explanatory
  - `licensor --version` Self-explanatory
  - `licensor list` List out all possible IDs.
  - `licensor [IDENTIFIER] --header` Renders the associated header to stdout
  - _**Maybe**_ `licensor wizard` which does a questionaire to help you figure out which license you should choose. Depends on how hard this is to do with my limited knowledge of rust 😅
- Man page. No idea how hard that'll be. Never made one of those before 😅
- ~~Add piping out to stdout support~~
- Improve target breadth
  - macOS versions from one year ago onwards
  - Ubuntu LTS version
  - Ubuntu versions from 1 year ago onwards
  - Windows 10 versions from 1 year ago onwards
- Improve error handling
  - Don't just throw when given an incorrect ID
  - Maybe give suggestions based off of input if no license found, eg. giving something like `MIY` suggests `MIT`, `BSD-2` suggests `BSD-2-Clause`
- Provide metadata
  - SPDX link to license
  - FSF Free/Libre?
  - OSI Approved?
  - Full name?
  - Permissions?
  - Deprecated?
  - See also
  - Header

### Not planned

- Adding file headers.
  - By adding stdout support alongside with `--header`, some `sed` magic should get you this.
