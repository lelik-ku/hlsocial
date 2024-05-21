import { useEffect, useState } from "react";
import { Button, Space } from "antd";
import NotifyStatus from "./Notify";
import { User } from "../api/v1";


export default function Profile() {
  const [profile, setProfile] = useState<User>({});

  const getUser = async () => {
    let user_id: string = localStorage.getItem("user_id") || "0";
    const response: User = await fetch('/v1/users/' + user_id)
    .then((response) => {
      NotifyStatus(response.status)
      return (response).json()
    })
    .catch(()=> {});

    setProfile(response);
  };

  return (
    <div>
      <Space direction="vertical">
        <Button onClick={getUser}>Get self profile</Button>
      </Space>
      <hr />
      {/* <Descriptions title="User Profile" bordered items={profile} /> */}
      <p>{JSON.stringify(profile, null, 2)}</p>
    </div>
  )
}
