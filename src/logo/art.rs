//! ASCII art for each supported logo, plus a generic fallback.
//!
//! Art is intentionally rectangular-ish; the renderer pads each line to the
//! widest one, so ragged right edges are fine. Keep lines free of trailing
//! whitespace and tabs.

pub const GENERIC: &str = r#"
   .--.
  |o_o |
  |:_/ |
 //   \ \
(|     | )
/'\_   _/`\
\___)=(___/
"#;

pub const ARCH: &str = r#"
      /\
     /  \
    /\   \
   /      \
  /   ,,   \
 /   |  |  -\
/_-''    ''-_\
"#;

pub const DEBIAN: &str = r#"
  _____
 /  __ \
|  /    |
|  \___-
-_
  --_
"#;

pub const FEDORA: &str = r#"
      _____
     /   __)\
     |  /  \ \
  ___|  |__/ /
 / (_    _)_/
/ /  |  |
\ \__/  |
 \(_____/
"#;

pub const UBUNTU: &str = r#"
         _
     ---(_)
 _/  ---  \
(_) |   |
  \  --- _/
     ---(_)
"#;

pub const WSL: &str = r#"
  /\_/\
 ( o.o )  WSL
  > ^ <
 /     \
( Linux )
 \_____/
"#;

pub const MAC: &str = r#"
     .:'
    _ :'_
 .'` `-' ``.
:          .-'
:         :
 :         `-;
  `.__.-.__.'
"#;