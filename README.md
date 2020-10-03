# Licensor

A bare minimum cli tool to generate a LICENSE file in the current directory. Make sure you don't already have a `LICENSE` file, and then just give it a valid [SPDX](https://spdx.org/licenses/) identifier and it'll put the license text into LICENSE file. It doesn't use the network to get the license information, it's all bundled in the binary.

### Example:

- `licensor Unlicense`
- `licensor 0BSD`

### Future Plans

- Add full CLI toolkit
  - `licensor --help` Self-explanatory
  - `licensor --version` Self-explanatory
  - `licensor [IDENTIFIER] --output [FILENAME]` Renders the output to a different filename than `LICENSE`
  - `licensor list` List out all possible IDs.
  - `licensor [IDENTIFIER] --header` Renders the associated header to stdout
- Man page. No idea how hard that'll be. Never made one of those before 😅
- Add piping out to stdout support
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
