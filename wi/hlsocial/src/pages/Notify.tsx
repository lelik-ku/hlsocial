import { notification } from "antd";


export function NotifySuccess(text: string) {
  notification.success({
    message: `Success`,
    description: text,
    placement: 'bottomRight'
  });
}

export function NotifyError(text: string) {
  notification.error({
    message: `Error`,
    description: text,
    placement: 'bottomRight'
  });
}
