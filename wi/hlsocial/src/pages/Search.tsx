import { Table, TableProps, Input, Space, notification, NotificationArgsProps } from "antd";
import { useCallback, useState } from "react";
import { User } from "../api/v1";
import { NotifyError, NotifySuccess } from "./Notify";
//import { Navigate } from "react-router-dom";

export default function Search() {
  const [msg, contextHolder] = notification.useNotification();
  type NotificationPlacement = NotificationArgsProps['placement'];

  const [search, setSearch] = useState();

  const searchUsers = async (text: String) => {
    const response = await fetch(encodeURI('/v1/search/' + text))
    .then((response) => {
      if (response.ok) {
        NotifySuccess("")
        return response.json()
      } else { 
        return response.text().then(text => { throw new Error(text) })
      };
    })
    .catch((e) => { NotifyError(e.toString()) });

    response && setSearch(response);
  };

  const onDelete = useCallback(async (user_id: string) => {
    try {
      await fetch (`/v1/users/${user_id}`, {method: 'DELETE'})
      .then((response) => {
        if (response.ok) {
          NotifySuccess("DELETE sucess")
        } else { 
          return response.text().then(text => { throw new Error(text) })
        };
      })
    } catch (e: any) { NotifyError(e.toString()) }
  }, [])

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
    {
      title: 'Action',
      key: 'action',
      render: (_: any, record: any) => (
        <Space size="middle">
          <a onClick={() => onDelete(record.user_id)}>Delete</a>
        </Space>
      ),
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

