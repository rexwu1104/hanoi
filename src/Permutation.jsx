import { useState } from "react";
import { Button, Offcanvas } from "react-bootstrap";
import LabeledNumberInput from "./LabeledNumberInput";
import LabeledStringInput from "./LabeledStringInput";
import getGlobalState from "./state";
import { invoke } from "@tauri-apps/api";

function Permutation() {
    let [show, setShow] = useState(false);
    let [contentValue, setContentValue] = useState('ABCD');
    let { n, k, setN, setK, setContent, sequence } = getGlobalState();

    let nChanger = (value) => {
        if (value > 26) value = 26;
        setN(value);
        setContent(contentValue = [...Array(value)].map((_, i) => String.fromCharCode(i + 65)).join(''));
        setContentValue(contentValue);
    };

    let contentChanger = (value) => {
        setContent(value);
        setN(value.length);
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', padding: 4 }}>
            <Button style={{ margin: 4 }} variant="outline-secondary" onClick={() => setShow(true)}>Permutation Settings</Button>
            <Button style={{ margin: 4 }} variant="outline-secondary" onClick={() => invoke('permutation')}>Permutation String</Button>
            <textarea style={{ height: window.outerHeight - 170, margin: 4 }} value={
                sequence.join('\n')
            } disabled></textarea>
            <Offcanvas style={{ width: 340 }} show={show} onHide={() => setShow(false)}>
                <Offcanvas.Header closeButton>
                    <Offcanvas.Title>Permutation Settings</Offcanvas.Title>
                </Offcanvas.Header>
                <Offcanvas.Body>
                    <LabeledNumberInput label="n = " value={n} onChange={nChanger} />
                    <LabeledNumberInput label="k = " value={k} onChange={setK} />
                    <LabeledStringInput label="string = " value={contentValue} onChange={setContentValue} onBlur={contentChanger} />
                </Offcanvas.Body>
            </Offcanvas>
        </div>
    );
}

export default Permutation;