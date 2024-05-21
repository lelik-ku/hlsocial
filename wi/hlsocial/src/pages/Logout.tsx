import { Button } from 'antd';
import NotifyStatus from './Notify';
// import { useState } from 'react';


export default function Logout() {
  // const [logout, setLogout] = useState();
  const callLogout = async () => {
    localStorage.clear();
    sessionStorage.clear();
    const response = await fetch('/v1/logout/', {method: 'POST'})
    .then((response) => {
      NotifyStatus(response.status)
      return response.json()      
    })
    .catch(()=> {});
  }

  return (
    <div>
      <b><h3>Press the button to logout</h3></b>
      <Button onClick={callLogout}>Logout</Button>
    </div>
  )
}
