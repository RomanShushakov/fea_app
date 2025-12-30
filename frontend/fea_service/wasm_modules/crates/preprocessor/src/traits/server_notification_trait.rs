use serde_json::Value;
use wasm_bindgen::JsValue;

use crate::props::Props;
use crate::enums::NotificationType;
use crate::functions::dispatch_custom_event;


pub trait ServerNotificationTrait
{
    type Key;
    fn get_event_detail(&self, notification_type: &NotificationType, key: Self::Key) -> Value;
    fn get_event_name(&self, notification_type: &NotificationType, props: &Props) -> String;
    fn get_event_target(&self, props: &Props) -> String;
    fn notify_server(&self, notification_type: NotificationType, key: Self::Key, props: &Props) -> Result<(), JsValue>
    {
        let detail = self.get_event_detail(&notification_type, key);
        let event_name = self.get_event_name(&notification_type, props);
        let event_target = self.get_event_target(props);
        dispatch_custom_event(detail, &event_name, &event_target)
    }
}
