import { Button, DatePicker, Form, Input, Layout, Select } from "antd";
import { useState } from 'react';
import { UserCreate } from "../api/v1";
import { NotifyError, NotifySuccess } from "./Notify";


const { Option } = Select;

export default function Register() {
  const [register, setRegister] = useState<UserCreate>();

  const callRegister = async (values: UserCreate) => {
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },      
      body: JSON.stringify(values)
    };
    const response: UserCreate = await fetch('/v1/register', requestOptions)
    .then((response) => {
      if (response.ok) {
        NotifySuccess("")
        return response.json()
      } else { 
        return response.text().then(text => { throw new Error(text) })
      };
    })
    .catch((e) => { NotifyError(e.toString()) });

    response && setRegister((response));
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
      onFinish={(values: UserCreate) => callRegister(values)}
      >
        <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
          <b><h3>Please enter login credentials:</h3></b>
        </Form.Item>

        <Form.Item<UserCreate>
        label="First name"
        name="first_name"
        rules={[{ required: true, message: 'Please input your first name!' }]}
        >
          <Input />
        </Form.Item>

        <Form.Item<UserCreate>
        label="Second name"
        name="second_name"
        rules={[{ required: false }]}
        >
          <Input />
        </Form.Item>

        <Form.Item<UserCreate>
        label="E-mail"
        name="email"
        rules={[{ required: true, message: 'Please input your e-mail!' }]}
        >
          <Input />
        </Form.Item>

        <Form.Item<UserCreate>
        label="Password"
        name="passwd"
        rules={[{ required: true, message: 'Please input your password!' }]}
        >
          <Input.Password />
        </Form.Item>

        <Form.Item<UserCreate>
        label="Gender"
        name="gender"
        rules={[{ required: false }]}
        >
          <Select placeholder="Gender" allowClear>
            <Option value="m">Male</Option>
            <Option value="f">Female</Option>
          </Select>
        </Form.Item>

        <Form.Item<UserCreate>
        label="Birthdate"
        name="birthdate"
        rules={[{ required: false }]}
        >
          <DatePicker format={"DD.MM.YYYY"} />
        </Form.Item>

        <Form.Item<UserCreate>
        label="Biography"
        name="biography"
        rules={[{ required: false }]}
        >
          <Input />
        </Form.Item>

        <Form.Item<UserCreate>
        label="City"
        name="city"
        rules={[{ required: false }]}
        >
          <Input />
        </Form.Item>

        <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
        </Form.Item>
      </Form>
      {register && <p>Result: {JSON.stringify(register, null, 2)}</p>}
    </Layout>
  )
}
