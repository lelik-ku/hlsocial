import { notification } from "antd";


export default function NotifyStatus(status: number) {
  if (status == 200) {
    NotifySuccess(status.toString())
  }
  else {
    NotifyError(status.toString())
  }
}

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
