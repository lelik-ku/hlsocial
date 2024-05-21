import { Table, TableProps, Input, Space, notification, NotificationArgsProps } from "antd";
import { useState } from "react";
import { User } from "../api/v1";
import NotifyStatus from "./Notify";
//import { Navigate } from "react-router-dom";

export default function Search() {
  const [msg, contextHolder] = notification.useNotification();
  type NotificationPlacement = NotificationArgsProps['placement'];

  const [search, setSearch] = useState();

  const searchUsers = async (text: String) => {
    const response = await fetch(encodeURI('/v1/search/' + text))
    .then((response) => {
      NotifyStatus(response.status)
      return response.json()
    })
    .catch(()=> {});

    setSearch(response);
  };

  const { Search } = Input;

  const users_columns: TableProps<User>['columns'] = [
    {
        title: 'User ID',
        dataIndex: 'user_id',
        key: 'user_id',
    },
    {
        title: 'First Name',
        dataIndex: 'first_name',
        key: 'first_name',
    },
    {
        title: 'Second Name',
        dataIndex: 'second_name',
        key: 'second_name',
    },
    {
        title: 'E-mail',
        dataIndex: 'email',
        key: 'email',
    },
    {
        title: 'Gender',
        dataIndex: 'gender',
        key: 'gender',
    },
    {
        title: 'Birthdate',
        dataIndex: 'birthdate',
        key: 'birthdate',
    },
    {
      title: 'City',
      dataIndex: 'city',
      key: 'city',
    },
  ];

  return (
    <>
    {contextHolder}
    <div>
      <Space direction="vertical">
        <Search placeholder="input search text" onSearch={searchUsers} allowClear style={{ width: 400 }} />
      </Space>
      <hr />
      <Table dataSource={search} columns={users_columns} rowKey={'user_id'}></Table>
    </div>
    </>
  )
}

