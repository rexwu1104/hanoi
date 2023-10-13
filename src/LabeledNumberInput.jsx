import { InputGroup } from "react-bootstrap";

function LabeledNumberInput({ label, value, onChange }) {
    let changer = (event) => {
        let v = event.target.value;
        if (/\d*/g.test(v) && !Number.isNaN(v = Number(v))) {
            onChange(v);
        } else {
            onChange(value);
        }
    };

    return (
        <InputGroup style={{ padding: 4 }}>
            <InputGroup.Text>{label}</InputGroup.Text>
            <input className="form-control" placeholder="please use integer" onChange={changer} value={value}></input>
        </InputGroup>
    );
}

export default LabeledNumberInput;