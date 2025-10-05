use super::*;

#[test]
fn it_accept_empty_strings_as_input() {
    assert_eq!(tokenize(""), vec![]);
}

#[test]
fn it_should_tokenize_string() {
    let expected = vec![
        Token {
            keyword: Keywords::Tag,
            raw: "@shopping".to_string(),
            line: 1,
            start: 0,
            end: 9,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: " ".to_string(),
            line: 1,
            start: 9,
            end: 10,
        },
        Token {
            keyword: Keywords::Tag,
            raw: "@regression".to_string(),
            line: 1,
            start: 10,
            end: 21,
        },
        Token {
            keyword: Keywords::Feature,
            raw: "Feature:".to_string(),
            line: 2,
            start: 0,
            end: 8,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: " ".to_string(),
            line: 2,
            start: 8,
            end: 9,
        },
        Token {
            keyword: Keywords::Description,
            raw: "Shopping Cart Management".to_string(),
            line: 2,
            start: 9,
            end: 33,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "  ".to_string(),
            line: 3,
            start: 0,
            end: 2,
        },
        Token {
            keyword: Keywords::Description,
            raw: "As an online shopper".to_string(),
            line: 3,
            start: 2,
            end: 22,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "  ".to_string(),
            line: 4,
            start: 0,
            end: 2,
        },
        Token {
            keyword: Keywords::Description,
            raw: "I want to manage items in my cart".to_string(),
            line: 4,
            start: 2,
            end: 35,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "  ".to_string(),
            line: 5,
            start: 0,
            end: 2,
        },
        Token {
            keyword: Keywords::Description,
            raw: "So that I can purchase products".to_string(),
            line: 5,
            start: 2,
            end: 33,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "  ".to_string(),
            line: 7,
            start: 0,
            end: 2,
        },
        Token {
            keyword: Keywords::Background,
            raw: "Background:".to_string(),
            line: 7,
            start: 2,
            end: 13,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 8,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::Given,
            raw: "Given ".to_string(),
            line: 8,
            start: 4,
            end: 10,
        },
        Token {
            keyword: Keywords::Description,
            raw: "the user is logged in".to_string(),
            line: 8,
            start: 10,
            end: 31,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 9,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::And,
            raw: "And ".to_string(),
            line: 9,
            start: 4,
            end: 8,
        },
        Token {
            keyword: Keywords::Description,
            raw: "the shopping cart is empty".to_string(),
            line: 9,
            start: 8,
            end: 34,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "  ".to_string(),
            line: 11,
            start: 0,
            end: 2,
        },
        Token {
            keyword: Keywords::Tag,
            raw: "@smoke".to_string(),
            line: 11,
            start: 2,
            end: 8,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "  ".to_string(),
            line: 12,
            start: 0,
            end: 2,
        },
        Token {
            keyword: Keywords::Scenario,
            raw: "Scenario:".to_string(),
            line: 12,
            start: 2,
            end: 11,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: " ".to_string(),
            line: 12,
            start: 11,
            end: 12,
        },
        Token {
            keyword: Keywords::Description,
            raw: "Add single item to cart".to_string(),
            line: 12,
            start: 12,
            end: 35,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 13,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::Given,
            raw: "Given ".to_string(),
            line: 13,
            start: 4,
            end: 10,
        },
        Token {
            keyword: Keywords::Description,
            raw: "I am viewing a product \"Laptop\"".to_string(),
            line: 13,
            start: 10,
            end: 41,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 14,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::When,
            raw: "When ".to_string(),
            line: 14,
            start: 4,
            end: 9,
        },
        Token {
            keyword: Keywords::Description,
            raw: "I click the \"Add to Cart\" button".to_string(),
            line: 14,
            start: 9,
            end: 41,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 15,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::Then,
            raw: "Then ".to_string(),
            line: 15,
            start: 4,
            end: 9,
        },
        Token {
            keyword: Keywords::Description,
            raw: "the cart should contain 1 item".to_string(),
            line: 15,
            start: 9,
            end: 39,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 16,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::And,
            raw: "And ".to_string(),
            line: 16,
            start: 4,
            end: 8,
        },
        Token {
            keyword: Keywords::Description,
            raw: "the cart total should be \"$999\"".to_string(),
            line: 16,
            start: 8,
            end: 39,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "  ".to_string(),
            line: 18,
            start: 0,
            end: 2,
        },
        Token {
            keyword: Keywords::ScenarioOutline,
            raw: "Scenario Outline:".to_string(),
            line: 18,
            start: 2,
            end: 19,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: " ".to_string(),
            line: 18,
            start: 19,
            end: 20,
        },
        Token {
            keyword: Keywords::Description,
            raw: "Add multiple items".to_string(),
            line: 18,
            start: 20,
            end: 38,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 19,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::When,
            raw: "When ".to_string(),
            line: 19,
            start: 4,
            end: 9,
        },
        Token {
            keyword: Keywords::Description,
            raw: "I add <quantity> \"<product>\" to the cart".to_string(),
            line: 19,
            start: 9,
            end: 49,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 20,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::Then,
            raw: "Then ".to_string(),
            line: 20,
            start: 4,
            end: 9,
        },
        Token {
            keyword: Keywords::Description,
            raw: "the cart should contain <quantity> items".to_string(),
            line: 20,
            start: 9,
            end: 49,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 21,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::And,
            raw: "And ".to_string(),
            line: 21,
            start: 4,
            end: 8,
        },
        Token {
            keyword: Keywords::Description,
            raw: "the subtotal should be \"<total>\"".to_string(),
            line: 21,
            start: 8,
            end: 40,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 23,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::Examples,
            raw: "Examples:".to_string(),
            line: 23,
            start: 4,
            end: 13,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "      ".to_string(),
            line: 24,
            start: 0,
            end: 6,
        },
        Token {
            keyword: Keywords::Description,
            raw: "| product | quantity | total   |".to_string(),
            line: 24,
            start: 6,
            end: 38,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "      ".to_string(),
            line: 25,
            start: 0,
            end: 6,
        },
        Token {
            keyword: Keywords::Description,
            raw: "| Laptop  | 2        | $1998   |".to_string(),
            line: 25,
            start: 6,
            end: 38,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "      ".to_string(),
            line: 26,
            start: 0,
            end: 6,
        },
        Token {
            keyword: Keywords::Description,
            raw: "| Mouse   | 5        | $125    |".to_string(),
            line: 26,
            start: 6,
            end: 38,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "      ".to_string(),
            line: 27,
            start: 0,
            end: 6,
        },
        Token {
            keyword: Keywords::Description,
            raw: "| Keyboard| 1        | $79     |".to_string(),
            line: 27,
            start: 6,
            end: 38,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "  ".to_string(),
            line: 29,
            start: 0,
            end: 2,
        },
        Token {
            keyword: Keywords::Tag,
            raw: "@negative".to_string(),
            line: 29,
            start: 2,
            end: 11,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "  ".to_string(),
            line: 30,
            start: 0,
            end: 2,
        },
        Token {
            keyword: Keywords::Scenario,
            raw: "Scenario:".to_string(),
            line: 30,
            start: 2,
            end: 11,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: " ".to_string(),
            line: 30,
            start: 11,
            end: 12,
        },
        Token {
            keyword: Keywords::Description,
            raw: "Cannot add out of stock item".to_string(),
            line: 30,
            start: 12,
            end: 40,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 31,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::Given,
            raw: "Given ".to_string(),
            line: 31,
            start: 4,
            end: 10,
        },
        Token {
            keyword: Keywords::Description,
            raw: "the product \"Rare Item\" is out of stock".to_string(),
            line: 31,
            start: 10,
            end: 49,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 32,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::When,
            raw: "When ".to_string(),
            line: 32,
            start: 4,
            end: 9,
        },
        Token {
            keyword: Keywords::Description,
            raw: "I try to add it to the cart".to_string(),
            line: 32,
            start: 9,
            end: 36,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 33,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::Then,
            raw: "Then ".to_string(),
            line: 33,
            start: 4,
            end: 9,
        },
        Token {
            keyword: Keywords::Description,
            raw: "I should see an error message".to_string(),
            line: 33,
            start: 9,
            end: 38,
        },
        Token {
            keyword: Keywords::Whitespace,
            raw: "    ".to_string(),
            line: 34,
            start: 0,
            end: 4,
        },
        Token {
            keyword: Keywords::But,
            raw: "But ".to_string(),
            line: 34,
            start: 4,
            end: 8,
        },
        Token {
            keyword: Keywords::Description,
            raw: "the cart should remain empty".to_string(),
            line: 34,
            start: 8,
            end: 36,
        },
    ];

    assert_eq!(
        tokenize(
            r#"@shopping @regression
Feature: Shopping Cart Management
  As an online shopper
  I want to manage items in my cart
  So that I can purchase products

  Background:
    Given the user is logged in
    And the shopping cart is empty

  @smoke
  Scenario: Add single item to cart
    Given I am viewing a product "Laptop"
    When I click the "Add to Cart" button
    Then the cart should contain 1 item
    And the cart total should be "$999"

  Scenario Outline: Add multiple items
    When I add <quantity> "<product>" to the cart
    Then the cart should contain <quantity> items
    And the subtotal should be "<total>"

    Examples:
      | product | quantity | total   |
      | Laptop  | 2        | $1998   |
      | Mouse   | 5        | $125    |
      | Keyboard| 1        | $79     |

  @negative
  Scenario: Cannot add out of stock item
    Given the product "Rare Item" is out of stock
    When I try to add it to the cart
    Then I should see an error message
    But the cart should remain empty
"#
        ),
        expected
    );
}
