import { createActor, certificate_backend } from "../../../.dfx/local/canisters/certificate_backend";

const backend = certificate_backend;

window.issue = async function () {
  const student = document.getElementById("student").value;
  const course = document.getElementById("course").value;
  const hash = await backend.issue_certificate(student, course);
  document.getElementById("output").innerText = "Certificate issued. Hash:\n" + toHex(hash);
};

window.verify = async function () {
  const hashInput = document.getElementById("hash").value;
  const hash = fromHex(hashInput.trim());
  const cert = await backend.verify_certificate(hash);
  document.getElementById("output").innerText = cert
    ? `Student: ${cert.student_name}\nCourse: ${cert.course_name}\nDate: ${new Date(cert.issue_date / 1_000_000).toLocaleString()}`
    : "Certificate not found.";
};

function toHex(bytes) {
  return Array.from(bytes).map(b => b.toString(16).padStart(2, "0")).join("");
}

function fromHex(hex) {
  return Uint8Array.from(hex.match(/.{1,2}/g).map(byte => parseInt(byte, 16)));
}
