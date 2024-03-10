import React from 'react';
import { TeamOutlined, UserOutlined, PoweroffOutlined, SearchOutlined } from '@ant-design/icons';
import { Flex, MenuProps, Table } from 'antd';
import { Layout, Menu, theme, Image } from 'antd';
import logo from './hl.svg';
import './App.css';
import { Route, Routes, useNavigate } from 'react-router-dom';

const { Header, Content, Sider } = Layout;

const menu_items: MenuProps['items'] = [
  {
    key: `/`,
    icon: React.createElement(UserOutlined),
    label: `Home`,
  },
  {
    key: `profile`,
    icon: React.createElement(UserOutlined),
    label: `Profile`,
  },
  {
    key: `search`,
    icon: React.createElement(SearchOutlined),
    label: `Search users`,
  },
  {
    key: `users`,
    icon: React.createElement(TeamOutlined),
    label: `All users`,
  },
  {
    key: `logout`,
    icon: React.createElement(PoweroffOutlined),
    label: `Logout`,
    danger: true,
  },
];

const users_columns = [
  {
    title: 'Name',
    dataIndex: 'name',
    key: 'name',
  },
  {
    title: 'Age',
    dataIndex: 'age',
    key: 'age',
  },
  {
    title: 'Address',
    dataIndex: 'address',
    key: 'address',
  },
];

function Data() {
  return <div>
    <Routes>
      <Route path="/" element={<div>Root</div>}></Route>
      <Route path="/profile" element={<div>Profile</div>}></Route>
      <Route path="/search" element={<div>Search</div>}></Route>
      <Route path="/users" element={<div>Users<Table dataSource={dataSource} columns={users_columns} /></div>}></Route>
    </Routes>
  </div>
}

const App: React.FC = () => {
  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();

  const navigate = useNavigate();

  return (
    <Layout>
      <Header className='App-header' style={{ display: 'flex', alignItems: 'left' }}>
      <Flex gap="small" align="center">
        <img width={100} src={logo} />
        Social Network
      </Flex>
      </Header>

    <Layout>
      <Sider width={200}>
        <Menu
          mode="inline"
          defaultSelectedKeys={['1']}
          defaultOpenKeys={['sub1']}
          style={{ height: '100%', borderRight: 0 }}
          items={menu_items}
          onClick={({key})=>{
            navigate(key);
          }

          }
        />
      </Sider>
    <Layout style={{ padding: '0 24px 24px' }}>
      <Content
        style={{
          padding: 24,
          margin: 0,
          minHeight: 280,
          background: colorBgContainer,
          borderRadius: borderRadiusLG,
        }}
      >
        <Data />
      </Content>
    </Layout>
    </Layout>
    </Layout>
  );
};

export default App;

const dataSource = [
  {
    key: '1',
    name: 'Mike',
    age: 32,
    address: '10 Downing Street',
  },
  {
    key: '2',
    name: 'John',
    age: 42,
    address: '10 Downing Street',
  },
];