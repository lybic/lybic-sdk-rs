use super::length::Length;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComputerUseAction {
    #[serde(rename = "mouse:click")]
    MouseClick {
        x: Length,
        y: Length,
        button: i32,
        #[serde(rename = "holdKey", skip_serializing_if = "Option::is_none")]
        hold_key: Option<String>,
        #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
        call_id: Option<String>,
    },
    #[serde(rename = "mouse:doubleClick")]
    MouseDoubleClick {
        x: Length,
        y: Length,
        button: i32,
        #[serde(rename = "holdKey", skip_serializing_if = "Option::is_none")]
        hold_key: Option<String>,
        #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
        call_id: Option<String>,
    },
    #[serde(rename = "mouse:move")]
    MouseMove {
        x: Length,
        y: Length,
        #[serde(rename = "holdKey", skip_serializing_if = "Option::is_none")]
        hold_key: Option<String>,
        #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
        call_id: Option<String>,
    },
    #[serde(rename = "mouse:scroll")]
    MouseScroll {
        x: Length,
        y: Length,
        #[serde(rename = "stepVertical")]
        step_vertical: i32,
        #[serde(rename = "stepHorizontal")]
        step_horizontal: i32,
        #[serde(rename = "holdKey", skip_serializing_if = "Option::is_none")]
        hold_key: Option<String>,
        #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
        call_id: Option<String>,
    },
    #[serde(rename = "mouse:drag")]
    MouseDrag {
        #[serde(rename = "startX")]
        start_x: Length,
        #[serde(rename = "startY")]
        start_y: Length,
        #[serde(rename = "endX")]
        end_x: Length,
        #[serde(rename = "endY")]
        end_y: Length,
        #[serde(rename = "holdKey", skip_serializing_if = "Option::is_none")]
        hold_key: Option<String>,
        #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
        call_id: Option<String>,
    },
    #[serde(rename = "keyboard:type")]
    KeyboardType {
        content: String,
        #[serde(rename = "treatNewLineAsEnter")]
        treat_new_line_as_enter: bool,
        #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
        call_id: Option<String>,
    },
    #[serde(rename = "keyboard:hotkey")]
    KeyboardHotkey {
        keys: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i32>,
        #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
        call_id: Option<String>,
    },
    #[serde(rename = "screenshot")]
    Screenshot {
        #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
        call_id: Option<String>,
    },
    #[serde(rename = "wait")]
    Wait {
        duration: i32,
        #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
        call_id: Option<String>,
    },
    #[serde(rename = "finished")]
    Finished {
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
        call_id: Option<String>,
    },
    #[serde(rename = "failed")]
    Failed {
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
        call_id: Option<String>,
    },
}
