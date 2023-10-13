import { useState } from "react";
import { Button, Offcanvas, ProgressBar, Spinner } from "react-bootstrap";
import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/shell";
import getGlobalState from "./state";
import LabeledNumberInput from "./LabeledNumberInput";

function Hanoi({ openChart }) {
    let [show, setShow] = useState(false);
    let [generating, setGenerating] = useState(false);
    let [complete, setComplete] = useState(false);
    let { rods, setRods, plates, setPlates, fps, setFps, max_frame, current_frame, video_path } = getGlobalState();

    let percent = (current_frame / max_frame * 100).toFixed(2);
    return (
        <div style={{ display: 'flex', flexDirection: 'column', padding: 4 }}>
            <Button style={{ margin: 4 }} variant="outline-secondary" onClick={() => setShow(true)}>Hanoi Settings</Button>
            <Button style={{ margin: 4 }} variant="outline-secondary" onClick={() => {
                setGenerating(true);
                invoke("hanoi").then(() => {
                    setComplete(true);
                    setGenerating(false)
                });
            }}>Generate Animation</Button>
            { generating ? <>
                <ProgressBar animated now={percent} label={`${percent}%`}></ProgressBar>
                <div style={{
                    padding: 20,
                    justifyContent: "center",
                    alignItems: "center",
                    display: 'flex',
                    flexDirection: 'column'
                }}>
                    <span>Generating animation...</span>
                    <Spinner
                        as="div"
                        animation="grow"
                        role="status"
                    ></Spinner>
                </div>
            </> : complete ? <>
                <ProgressBar animated now={percent} label={`${percent}%`}></ProgressBar>
                <div style={{
                    padding: 20,
                    justifyContent: "center",
                    alignItems: "center",
                    display: 'flex',
                    flexDirection: 'column'
                }}>
                    <span>Generating animation completed.</span>
                </div>
                <Button variant="outline-secondary" onClick={async () => await open(video_path)}>Open animation</Button>
            </> : <></> }
            <Offcanvas style={{ width: 340 }} show={show} onHide={() => setShow(false)}>
                <Offcanvas.Header closeButton>
                    <Offcanvas.Title>Hanoi Settings</Offcanvas.Title>
                </Offcanvas.Header>
                <Offcanvas.Body>
                    <LabeledNumberInput label="plates = " value={plates} onChange={setPlates} />
                    <LabeledNumberInput label="rods = " value={rods} onChange={setRods} />
                    <LabeledNumberInput label="fps = " value={fps} onChange={setFps} />
                </Offcanvas.Body>
            </Offcanvas>
        </div>
    );
}

export default Hanoi;