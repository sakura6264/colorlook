// No need to import egui directly
use egui_toast::{Toast, ToastKind, ToastOptions, ToastStyle};

/// Creates a success toast notification
pub fn success(message: impl Into<String>) -> Toast {
    Toast {
        kind: ToastKind::Success,
        text: message.into().into(),
        options: ToastOptions::default()
            .duration_in_seconds(2.0)
            .show_progress(true),
        style: ToastStyle::default(),
    }
}

/// Creates an error toast notification
pub fn error(message: impl Into<String>) -> Toast {
    Toast {
        kind: ToastKind::Error,
        text: message.into().into(),
        options: ToastOptions::default()
            .duration_in_seconds(5.0)
            .show_progress(true),
        style: ToastStyle::default(),
    }
}

/// Helper function to handle results with toast notifications
pub fn handle_result<T, E: std::fmt::Display>(
    result: Result<T, E>,
    success_msg: impl Into<String>,
    error_prefix: impl Into<String>,
    toasts: &mut egui_toast::Toasts,
) -> Option<T> {
    match result {
        Ok(value) => {
            toasts.add(success(success_msg));
            Some(value)
        }
        Err(err) => {
            let prefix = error_prefix.into();
            toasts.add(error(format!("{}: {}", prefix, err)));
            None
        }
    }
}
