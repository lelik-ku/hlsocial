import { Button, Form, Input, Layout, Space, Table, TableProps } from "antd";
import { useEffect, useState } from 'react';
import { UserLoginByEmail, UserLoginResult } from "../api/v1";
import NotifyStatus from "./Notify";


const userid = "user_id";

export default function Login() {
  const [login, setLogin] = useState(localStorage.getItem(userid) || null);

  const callLogin = async (email: string, pass: string) => {
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email: email, passwd: pass })
    };
    const response = await fetch('/v1/login/', requestOptions)
    .then((response) => {
      NotifyStatus(response.status)
      return response.json()
    })
    .catch(()=> {});

    localStorage.setItem(userid, response[userid]);
    setLogin(JSON.stringify(response));
  }
  
  return (
    <Layout>
      <Form
      name="basic"
      labelCol={{ span: 8 }}
      wrapperCol={{ span: 16 }}    
      style={{ maxWidth: 600 }}
      initialValues={{ remember: true }}
      autoComplete="off"
      onFinish={(values) => callLogin(values.email, values.passwd)}
      >
        <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
          <b><h3>Please enter login credentials:</h3></b>
        </Form.Item>

        <Form.Item<UserLoginByEmail>
        label="E-mail"
        name="email"
        rules={[{ required: true, message: 'Please input your username!' }]}
        >
          <Input />
        </Form.Item>

        <Form.Item<UserLoginByEmail>
        label="Password"
        name="passwd"
        rules={[{ required: true, message: 'Please input your password!' }]}
        >
          <Input.Password />
        </Form.Item>
        <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
        </Form.Item>
      </Form>
      {login && <p>Result: {login}</p>}
    </Layout>
  )
}
