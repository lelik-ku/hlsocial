import { Button, Form, Input } from "antd";
import { useState } from 'react';


type FieldType = {
    email?: string;
    password?: string;
  };

export default function Login() {
    const [login, setLogin] = useState();
    const callLogin = async (email: string, pass: string) => {
        const requestOptions = {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email: email, passwd: pass })
        };
        const response = await fetch('/v1/login/', requestOptions)
        .then((response) => response.json())
        .catch(()=> {});

        setLogin(response);
    }

    return (
        <Form
        name="basic"
        labelCol={{ span: 8 }}
        wrapperCol={{ span: 16 }}    
        style={{ maxWidth: 600 }}
        initialValues={{ remember: true }}
        autoComplete="off"
        onFinish={(values) => callLogin(values.email, values.password)}
        >
            <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
                <b>Please enter login credentials:</b>
            </Form.Item>

            <Form.Item<FieldType>
            label="E-mail"
            name="email"
            rules={[{ required: true, message: 'Please input your username!' }]}
            >
                <Input />
            </Form.Item>

            <Form.Item<FieldType>
            label="Password"
            name="password"
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
    )
}
