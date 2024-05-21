import { Button } from 'antd';
import { NotifyError, NotifySuccess } from "./Notify";
// import { useState } from 'react';


export default function Logout() {
  // const [logout, setLogout] = useState();
  const callLogout = async () => {
    localStorage.clear();
    sessionStorage.clear();
    const _ = await fetch('/v1/logout/', {method: 'POST'})
    .then((response) => {
      if (response.ok) {
        NotifySuccess("")
      } else { 
        return response.text().then(text => { throw new Error(text) })
      };
      return
    })
    .catch((e) => { NotifyError(e.toString()) });
  }

  return (
    <div>
      <b><h3>Press the button to logout</h3></b>
      <Button onClick={callLogout}>Logout</Button>
    </div>
  )
}
