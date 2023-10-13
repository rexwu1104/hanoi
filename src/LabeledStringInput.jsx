import { InputGroup } from "react-bootstrap";

function LabeledStringInput({ label, value, onChange, onBlur }) {
    let changer = (v) => {
        onChange(v.target.value);
    };

    let blurer = (v) => {
        onBlur(v.target.value);
    };

    return (
        <InputGroup style={{ padding: 4 }}>
            <InputGroup.Text>{label}</InputGroup.Text>
            <input className="form-control" placeholder="content" onChange={changer} onBlur={blurer} value={value}></input>
        </InputGroup>
    );
}

export default LabeledStringInput;