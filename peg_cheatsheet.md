- Use () to crate internal structure in patterns

- SOI
  - Start of Input to parse

- EOI
  - End of Input to parse

- WHITESPACE
  - A silent rule that implicitly ignores whitespace characters at every ~, +, and *

- LHS ~ RHS
  - signifies that LHS must come before RHS

- (PATTERN)*
  - PATTERN can repeat 0 or more times

- (PATTERN)+
  - PATTERN can repeat 1 or more times

- A Pattern can be a literal or rules