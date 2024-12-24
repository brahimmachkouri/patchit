# Patch a file or create a patch 

Applies a patch to a file using a JSON file that contains the name of the file to patch with its checksum, along with the offsets and the byte data to be modified.
That JSON patch file is created by the tool itself.

```bash
Usage: patchit [OPTIONS]

Options:
  --source, -s <file>    Path of the source/original file (generate mode)
  --modified, -m <file>  Path of the modified file (generate mode)
  --output, -o <file>    Name of the output JSON file (default: modified.json)
  --help, -h             Display this help message

Examples (macOs/Linux):
  ./patchit --source original --modified modified [--output mypatch.json]
  ./patchit -s original -m modified [-o mypatch.json]
  ./patchit mypatch.json

Example on Windows:
  .\patchit.exe -s gimp.orig.exe -m gimp.exe [-o gimp.json]
```

Example of generated JSON file:
```json
	{
	  "file_name": "mybinary.exe",
	  "checksum": "79935e89d59728ac456b592ca7b4f64dee0f3a7bb10e44e068cf0c635f885735",
	  "patches": [
	    {
	      "offset": 190577,
	      "data": "75"
	    },
	    {
	      "offset": 1139552,
	      "data": "31"
	    },
	    {
	      "offset": 1139553,
	      "data": "c0"
	    },
	    {
	      "offset": 1139554,
	      "data": "c3"
	    }
	  ]
	}
```

Copyright (c) 2024 Brahim Machkouri

This software is provided "as is", without any warranty of any kind, express or implied, including but not limited to the warranties of merchantability and fitness for a particular purpose. In no event shall the author or copyright holders be liable for any damage, whether in an action of contract, tort, or otherwise, arising from the use of this software.
