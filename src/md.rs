enum Token {
  Text(String),
  Heading(String, u8),
  Bold(String),
  Italic(String),
  Code(String),
  Link(String, String),
  Image(String, String),
  List(Vec<String>, bool),
  Quote(Vec<String>),
  Hr,
}

struct Parser<'a> {
  input: &'a str,
  pos: usize,
}

impl<'a> Parser<'a> {
  fn new(input: &'a str) -> Parser<'a> {
      Parser { input, pos: 0 }
  }

  fn parse(&mut self) -> Vec<Token> {
      let mut tokens = Vec::new();
      while self.pos < self.input.len() {
          match self.input.as_bytes()[self.pos] {
              b'#' => tokens.push(self.parse_heading()),
              b'*' => tokens.push(self.parse_bold_or_italic()),
              b'_' => tokens.push(self.parse_bold_or_italic()),
              b'`' => tokens.push(self.parse_code()),
              b'[' => tokens.push(self.parse_link_or_image()),
              b'!' => {
                  self.consume();
                  tokens.push(Token::Image(self.parse_link_text(), self.parse_link_url()));
              }
              b'-' => {
                  self.consume();
                  if self.consume() == b' ' {
                      tokens.push(self.parse_list());
                  } else {
                      self.pos -= 1;
                      tokens.push(Token::Text("-".to_string()));
                  }
              }
              b'>' => tokens.push(self.parse_quote()),
              b'\n' => {
                  self.consume();
                  tokens.push(Token::Text("\n".to_string()));
              }
              b'\r' => {
                  self.consume();
              }
              b'\t' | b' ' => {
                  self.consume();
              }
              b'_' => {
                  self.consume();
                  tokens.push(self.parse_italic());
              }
              _ => tokens.push(Token::Text(self.parse_text())),
          }
      }
      tokens
  }

  fn parse_heading(&mut self) -> Token {
      let mut level = 0;
      while self.consume() == b'#' {
          level += 1;
      }
      while self.consume() == b' ' {}
      let text = self.parse_text();
      Token::Heading(text, level)
  }

  fn parse_bold_or_italic(&mut self) -> Token {
      let start = self.consume();
      let end = if start == b'*' { b'*' } else { b'_' };
      let text = self.parse_text();
      if self.consume() == end && !text.is_empty() {
          if start == b'*' {
              Token::Bold(text)
          } else {
              Token::Italic(text)
          }
      } else {
          Token::Text(format!("{}{}", start as char, text))
      }
  }

  fn parse_code(&mut self) -> Token {
      self.consume();
      let text = self.parse_text();
      if self.consume() == b'`' && !text.is_empty() {
          Token::Code(text)
      } else {
          Token::Text(format!("`{}`", text))
      }
  }

  fn parse_link_or_image(&mut self) -> Token {
      self.consume();
      let text = self.parse_link_text();
      if self.consume() == b']' && self.consume() == b'(' {
          let url = self.parse_link_url();
          if self.consume() == b')' {
              if text.starts_with('!') {
                  Token::Image(text[1..].to_string(), url)
              } else {
                  Token::Link(text, url)
              }
          } else {
              Token::Text(format!("[{}({}", text, url))
          }
      } else {
          Token::Text(format!("[{}", text))
      }
  }

  fn parse_link_text(&mut self) -> String {
      let mut text = String::new();
      while let Some(c) = self.input[self.pos..].chars().next() {
          match c {
              ']' => break,
              '\\' => {
                  self.consume();
                  if let Some(c) = self.input[self.pos..].chars().next() {
                      text.push(c);
                  }
              }
              _ => text.push(c),
          }
          self.consume();
      }
      text
  }

  fn parse_link_url(&mut self) -> String {
      let mut url = String::new();
      while let Some(c) = self.input[self.pos..].chars().next() {
          match c {
              ')' => break,
              '\\' => {
                  self.consume();
                  if let Some(c) = self.input[self.pos..].chars().next() {
                      url.push(c);
                  }
              }
              _ => url.push(c),
          }
          self.consume();
      }
      url
  }

  fn parse_list(&mut self) -> Token {
      let mut items = Vec::new();
      items.push(self.parse_list_item());
      while let Some(c) = self.input[self.pos..].chars().next() {
          match c {
              '\n' => {
                  self.consume();
                  if let Some(c) = self.input[self.pos..].chars().next() {
                      if c == '\r' || c == '\n' || c == '\t' || c == ' ' {
                          self.consume();
                          items.push(self.parse_list_item());
                      } else {
                          break;
                      }
                  } else {
                      break;
                  }
              }
              _ => break,
          }
      }
      Token::List(items, false)
  }

  fn parse_list_item(&mut self) -> String {
      let mut text = String::new();
      while let Some(c) = self.input[self.pos..].chars().next() {
          match c {
              '\n' => break,
              '\r' => {
                  self.consume();
                  if let Some(c) = self.input[self.pos..].chars().next() {
                      if c == '\n' {
                          break;
                      }
                  }
              }
              _ => {
                  text.push(c);
                  self.consume();
              }
          }
      }
      text
  }

  fn parse_quote(&mut self) -> Token {
      let mut lines = Vec::new();
      while let Some(c) = self.input[self.pos..].chars().next() {
          match c {
              '\n' => {
                  self.consume();
                  lines.push(self.parse_text());
              }
              '\r' => {
                  self.consume();
              }
              '>' => {
                  self.consume();
                  if self.consume() == b' ' {
                      lines.push(self.parse_text());
                  } else {
                      self.pos -= 1;
                      break;
                  }
              }
              _ => break,
          }
      }
      Token::Quote(lines)
  }

  fn parse_italic(&mut self) -> Token {
      let text = self.parse_text();
      if self.consume() == b'_' && !text.is_empty() {
          Token::Italic(text)
      } else {
          Token::Text(format!("*{}*", text))
      }
  }

  fn parse_text(&mut self) -> String {
      let mut text = String::new();
      while let Some(c) = self.input[self.pos..].chars().next() {
          match c {
              '#' | '*' | '_' | '`' | '[' | '!' | '-' | '>' | '\n' | '\r' => break,
              '\\' => {
                  self.consume();
                  if let Some(c) = self.input[self.pos..].chars().next() {
                      text.push(c);
                  }
              }
              _ => {
                  text.push(c);
                  self.consume();
              }
          }
      }
      text
  }

  fn consume(&mut self) -> u8 {
      let b = self.input.as_bytes()[self.pos];
      self.pos += 1;
      b
  }
}

#[derive(Debug)]
enum AstNode {
  Text(String),
  Heading(String, u8),
  Bold(Vec<AstNode>),
  Italic(Vec<AstNode>),
  Code(String),
  Link(String, String),
  Image(String, String),
  List(Vec<Vec<AstNode>>, bool),
  Quote(Vec<Vec<AstNode>>),
  Hr,
}

fn build_ast(tokens: &[Token]) -> Vec<AstNode> {
  let mut nodes = Vec::new();
  let mut i = 0;
  while i < tokens.len() {
      match tokens[i] {
          Token::Text(ref text) => {
              nodes.push(AstNode::Text(text.clone()));
              i += 1;
          }
          Token::Heading(ref text, level) => {
              nodes.push(AstNode::Heading(text.clone(), level));
              i += 1;
          }
          Token::Bold(ref text) => {
              nodes.push(AstNode::Bold(vec![AstNode::Text(text.clone())]));
              i += 1;
          }
          Token::Italic(ref text) => {
              nodes.push(AstNode::Italic(vec![AstNode::Text(text.clone())]));
              i += 1;
          }
          Token::Code(ref text) => {
              nodes.push(AstNode::Code(text.clone()));
              i += 1;
          }
          Token::Link(ref text, ref url) => {
              nodes.push(AstNode::Link(text.clone(), url.clone()));
              i += 1;
          }
          Token::Image(ref text, ref url) => {
              nodes.push(AstNode::Image(text.clone(), url.clone()));
              i += 1;
          }
          Token::List(ref items, ordered) => {
              let mut nodes_list = Vec::new();
              for item in items {
                  nodes_list.push(build_ast(&parse_text(item)));
              }
              nodes.push(AstNode::List(nodes_list, ordered));
              i += 1;
          }
          Token::Quote(ref lines) => {
              let mut nodes_list = Vec::new();
              for line in lines {
                  nodes_list.push(build_ast(&parse_text(line)));
              }
              nodes.push(AstNode::Quote(nodes_list));
              i += 1;
          }
          Token::Hr => {
              nodes.push(AstNode::Hr);
              i += 1;
          }
      }
  }
  nodes
}

fn parse_text(text: &str) -> Vec<Token> {
  let mut parser = Parser::new(text);
  parser.parse()
}

fn main() {
  let markdown_input = "# Hello, world!\n\nThis is a **Markdown** document.";
  let tokens = parse_text(markdown_input);
  let ast = build_ast(&tokens);
  println!("{:#?}", ast);
}
