import React from 'react';
import { Route, Routes, useNavigate } from 'react-router-dom';
import { TeamOutlined, UserOutlined, PoweroffOutlined, SearchOutlined, HomeOutlined } from '@ant-design/icons';
import { Flex, MenuProps } from 'antd';
import { Layout, Menu, theme } from 'antd';
import logo from './hl.svg';
import './App.css';
import Profile from './pages/Profile';
import Users from './pages/Users';
import Search from './pages/Search';
import Logout from './pages/Logout';
import Login from './pages/Login';

const { Header, Content, Sider } = Layout;

const menu_items: MenuProps['items'] = [
  {
    key: `/`,
    icon: React.createElement(HomeOutlined),
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
  },
];

function MenuItems() {
  return <div>
    <Routes>
      <Route path="/" element={<div>Welcome to HL Social Network!</div>}></Route>
      <Route path="/login" element={Login()}></Route>
      <Route path="/profile" element={Profile()}></Route>
      <Route path="/search" element={Search()}></Route>
      <Route path="/users" element={Users()}></Route>
      <Route path="/logout" element={Logout()}></Route>
    </Routes>
  </div>
}

function IsLoggedIn() {
  return false;
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
        <MenuItems />
      </Content>
    </Layout>
    </Layout>
    </Layout>
  );
};

export default App;
