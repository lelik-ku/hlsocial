import { Button, Space, Table } from "antd";
import { useCallback, useState } from "react";

import { NotifyError, NotifySuccess } from "./Notify";

export default function AllUsers() {
  const [users, setUsers] = useState();

  const getUsers = async () => {
    const response = await fetch('/v1/users')
    .then((response) => {
      if (response.ok) {
        NotifySuccess("")
        return response.json()
      } else { 
        return response.text().then(text => { throw new Error(text) })
      };
    })
    .catch((e)=> { NotifyError(e.toString()) });

    setUsers(response);
  };

  const onDelete = useCallback(async (user_id: string) => {
    try {
      await fetch (`/v1/users/${user_id}`, {method: 'DELETE'})
      .then((response) => {
        if (response.ok) {
          NotifySuccess("DELETE success")
          getUsers()
        } else { 
          return response.text().then(text => { throw new Error(text) })
        };
      })
    } catch (e: any) { NotifyError(e.toString()) }
  }, [])

  const users_columns = [
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
    <div>
      <Space direction="vertical">
        <Button onClick={getUsers}>Get all users</Button>
      </Space>
      <hr />
      <Table dataSource={users} columns={users_columns} rowKey={'user_id'}></Table>
    </div>
  )
}
