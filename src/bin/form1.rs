#![windows_subsystem = "windows"]

use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    qt_core::QString,
    qt_core::SlotNoArgs,
    QApplication, QLineEdit, QMessageBox, QPushButton, QVBoxLayout, QWidget,
};

struct Form {
    _widget: CppBox<QWidget>,
    _button: MutPtr<QPushButton>,
    _line_edit: MutPtr<QLineEdit>,
    button_clicked: CppBox<SlotNoArgs>,
    line_edit_edited: CppBox<SlotNoArgs>,
}

impl Form {
    fn new() -> Form {
        unsafe {
            let mut widget = QWidget::new_0a();
            let mut layout = QVBoxLayout::new_1a(&mut widget).into_ptr();
            let mut line_edit = QLineEdit::new();

            layout.add_widget(&mut line_edit);
            let line_edit = line_edit.into_ptr();

            let mut button = QPushButton::from_q_string(&QString::from_std_str("Start"));
            button.set_enabled(false);

            layout.add_widget(&mut button);
            let mut button = button.into_ptr();

            widget.show();
            let widget_ptr = widget.as_mut_ptr();

            let form = Form {
                button_clicked: SlotNoArgs::with(move || {
                    let text = line_edit.text();
                    QMessageBox::information_q_widget2_q_string(
                        widget_ptr,
                        &QString::from_std_str("My title"),
                        &QString::from_std_str("Text: \"%1\". Congratulations!")
                            .arg_q_string(&text),
                    );
                }),
                line_edit_edited: SlotNoArgs::with(move || {
                    button.set_enabled(!line_edit.text().is_empty());
                }),
                _widget: widget,
                _button: button,
                _line_edit: line_edit,
            };
            button.clicked().connect(&form.button_clicked);
            line_edit.text_edited().connect(&form.line_edit_edited);
            form
        }
    }
}

fn main() {
    QApplication::init(|_| unsafe {
        let _form = Form::new();
        QApplication::exec()
    })
}
