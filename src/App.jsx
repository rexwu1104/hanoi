import { useState, useEffect } from "react";
import { Tabs, Tab } from "react-bootstrap";
import { subscribeState } from "./state";
import Permutation from "./Permutation";
import Hanoi from "./Hanoi";

function App() {
    let [activeKey, setActiveKey] = useState("home");
    useEffect(() => {
        const unsubscribeStateSync = subscribeState();
        return () => {
            unsubscribeStateSync.then((unsubscribe) => unsubscribe());
        };
    }, []);

    return (
        <Tabs
            defaultActiveKey="home"
            id="uncontrolled-tab-example"
            className="mb-3"
            onSelect={(v) => setActiveKey(v)}
            activeKey={activeKey}
        >
            <Tab eventKey="home" title="Permutation">
                <Permutation />
            </Tab>
            <Tab eventKey="hanoi" title="Hanoi">
                <Hanoi />
            </Tab>
        </Tabs>
    );
}

export default App;
