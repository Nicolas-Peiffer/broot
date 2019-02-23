use crate::shell_install::ShellFamily;

pub const FISH: ShellFamily<'static> = ShellFamily {
    name: "fish",
    sourcing_files: &[".config/fish/config.fish"], // idealy we should probably use XDG here...
    version: 1,
    script: FISH_FUNC
};

const FISH_FUNC: &str = r#"
# This script was automatically generated by the broot function
# More information can be found in https://github.com/Canop/broot/blob/master/documentation.md

# This function starts broot and executes the command
# it produces, if any.
# It's needed because some shell commands, like `cd`,
# have no useful effect if executed in a subshell.
function br
    set f (mktemp)
    broot --outcmd $f $argv
    if test $status -ne 0
        rm -f "$f"
        return "$code"
    end
    set d (cat "$f")
    rm -f "$f"
    eval "$d"
end
"#;
