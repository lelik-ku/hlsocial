import { Button, Space, Table } from "antd";
import { useState } from "react";

export default function AllUsers() {
  const [users, setUsers] = useState();

  const getUsers = async () => {
    const response = await fetch('/v1/users')
    .then((response) => response.json())
    .catch(()=> {});

    setUsers(response);
  };

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
