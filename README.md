# egui_css: A Styling Framework for egui (Very very very WIP)

**egui_css** is an extension for [egui](https://github.com/emilk/egui), a Rust-based immediate mode GUI library. This framework introduces a cascading style sheet (CSS) approach to styling, allowing developers to define styles declaratively for their egui applications.

## Features

- **Blazing Fast:** The css parser is blazing fast
- **CSS-Like Syntax:** Write styles in a familiar CSS-inspired format, enabling intuitive and organized customization of your UI.

## Missing Features

- Layout system, a DOM is required for that, so probably is a good idea if egui_css provide this solution with a BTree 🤔
- Media queries for responsive design.
- Advanced selectors like `:hover`, `:focus`, or pseudo-elements. (implemented but not works 😔)
- Animation definitions within stylesheets.
- Some issues with increasing memory 🫠

## Supported CSS Rules

The following CSS-like rules are supported:

- `width`: Sets the width of an element.
- `height`: Sets the height of an element.
- `min-width`: Sets the minimum width of an element.
- `min-height`: Sets the minimum height of an element.
- `max-width`: Sets the maximum width of an element.
- `max-height`: Sets the maximum height of an element.
- `border-color`: Sets the border color.
- `border`: Sets the border width and color.
- `border-width`: Sets the width of the border.
- `border-radius`: Sets the rounding of corners.
- `padding`: Defines the padding around elements.
- `padding-left`: Sets the left padding.
- `padding-top`: Sets the top padding.
- `padding-right`: Sets the right padding.
- `padding-bottom`: Sets the bottom padding.
- `margin`: Defines the margin around elements.
- `margin-left`: Sets the left margin.
- `margin-top`: Sets the top margin.
- `margin-right`: Sets the right margin.
- `margin-bottom`: Sets the bottom margin.
- `cursor`: Defines the type of cursor to display.
- `cursor-color`: Sets the cursor color.
- `color`: Sets the text color.
- `background-color`: Sets the background color.
- `box-shadow`: Sets the shadow around elements.
- `column-gap`: Sets the gap between columns.
- `row-gap`: Sets the gap between rows.
- `gap`: Sets the gap between rows and columns.
- `text-overflow`: Defines how text overflows.
- `transition`: Sets the transition duration for animations. (no animate)

## TODO List of Properties

The following properties are planned for future support:

- `display`: Define how elements are displayed (e.g., block, inline).
- `position`: Control the positioning of elements.
- `flex-direction`: Define the main axis of flex container elements.
- `flex-wrap`: Allow wrapping of flex items.
- `flex-grow`: Define the grow factor for flex items.
- `flex-shrink`: Define the shrink factor for flex items.
- `flex-basis`: Define the initial size of a flex item.
- `justify-content`: Align flex items along the main axis.
- `justify-self`: Align individual items inside a container.
- `align-self`: Align a specific item inside a flex container.
- `align-items`: Align items inside a flex container.
- `align-content`: Align multiple lines inside a flex container.
- `user-select`: Control text selection behavior.
- `z-index`: Control the stack order of elements.
- `aspect-ratio`: Maintain a specific aspect ratio for elements.
- `inset`: Control positioning inside a container (e.g., top, left).
- `outline`: Define the outline of elements.
- `font-size`: Set the size of the font.
- `font-family`: Specify the font family.
- `font-weight`: Define the weight of the font.
- `font-style`: Define the style of the font (e.g., italic).
- `line-height`: Set the height of lines of text.
- Your suggestion property 🎉

## Getting Started

### Prerequisites

- Rust 1.80 or later with the `cargo` build tool.
- Familiarity with egui basics (see the [egui documentation](https://docs.rs/egui/latest/egui/)).

### Usage

1. **Define a Style:**

```rust
egui_css::change_style(egui_css::StyleSheet::from_css(
    r#"
    .header {
      color: purple;
      padding: 20px;
      background-color: black;
      height: 50px;
    }

    #counter {
      color: green;
      background-color: white;
      padding: 20px;
      margin: 20px;
    }

    #counter:hover {
      color: purple;
    }

    .header:hover {
      color: blue;
    }
"#,
));
```

2. **Apply the StyleSheet:**

```rust
use egui_css::StyledWidgetExt;

ui.vertical(|ui| {
    ui.add(Label::new("My egui Application").styled().class(".header"));
    if ui
        .add(Button::new("Increment").styled().css_id("counter"))
        .clicked()
    {
        self.age += 1;
    }
    ui.add(
        Label::new(format!("age {}", self.age))
            .styled()
            .class(".text-orange-700"),
    );
    ui.add(Label::new("No styled"));
})
```

## Examples

Check out the examples in the `examples/` directory of the repository for more advanced use cases, including:

Run the examples with:

```bash
cargo run -p egui_css --example simple
```

## Contributing

We welcome contributions! Whether you have ideas for new features, bug fixes, or documentation improvements, feel free to open an issue or submit a pull request.

### How to Contribute

1. Fork the repository.
2. Create a branch for your feature or fix.
3. Open a pull request with a detailed description of the changes.

### Join Us

Help us expand **egui_css**! If you're interested in contributing, here are some ways to get involved:

- Add support for missing CSS properties.
- Improve the documentation and examples.
- Create issues for features or bugs you encounter.
- Share your feedback and suggestions for improvement.

Join us in shaping the future of egui_css and making styling in egui applications more flexible and powerful!
