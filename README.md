# Color

```color``` is a small (almost useless !) utility tool written in Rust designed to color a given pattern in a textual output.

## Usage
'''shell
cat greetings.txt | color hello --red
'''

It also support regular expressions (Rust style) :
'''shell
cat greetings.txt | color "[wW]o[rR]l[dD]" --cyan
'''

## limitations

'''more''' or '''less''' don't seem to color output so the utility is not very useful when it comes to display a medium length.

## Implementation

It is implemented in Rust (yes, you can do it with sed !) and is using the clap and regex libraries.
