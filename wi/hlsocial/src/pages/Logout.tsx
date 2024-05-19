import { Button } from 'antd';
import { useState } from 'react';


export default function Logout() {
    const [logout, setLogout] = useState();
    const callLogout = async () => {
        sessionStorage.clear();
        const response = await fetch('/v1/logout/', {method: 'POST'})
        .then((response) => response.json())
        .catch(()=> {});

        setLogout(response);
    }

    return (
        <div>
            <Button onClick={callLogout}>Logout</Button>
        </div>
    )
}
