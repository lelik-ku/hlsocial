import { useState } from "react";
import { Button, Descriptions, Space } from "antd";
import { DescriptionsItemType } from "antd/es/descriptions";

import { NotifyError, NotifySuccess } from "./Notify";
import { User } from "../api/v1";


export default function Profile() {
  const [profile, setProfile] = useState([] as DescriptionsItemType[]);

  const getUser = async () => {
    let user_id: string = localStorage.getItem("user_id") || "0";
    const response: User = await fetch(`/v1/users/${user_id}`)
    .then((response) => {
      if (response.ok) {
        NotifySuccess("")
        return response.json()
      } else { 
        return response.text().then(text => { throw new Error(text) })
      };
    })
    .catch((e) => { NotifyError(e.toString()) });

    response && setProfile(Object.entries(response).map(([k, v]) => ({ key: k, label: k, children: v })));
  };

  return (
    <div>
      <Space direction="vertical">
        <Button onClick={getUser}>Get self profile</Button>
      </Space>
      <hr />
      {profile && <Descriptions title="User Profile" bordered items={profile} />}
    </div>
  )
}
