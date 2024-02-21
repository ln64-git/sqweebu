import { invoke } from "@tauri-apps/api";

const handleResume = () => {
  invoke("resume_playback_from_frontend")
    .then((response) => console.log(response))
    .catch((error) => console.error(error));
};
const handlePause = () => {
  invoke("pause_playback_from_frontend")
    .then((response) => console.log(response))
    .catch((error) => console.error(error));
};
const handleStop = () => {
  invoke("stop_playback_from_frontend")
    .then((response) => console.log(response))
    .catch((error) => console.error(error));
};
const handleFastForward = () => {
  invoke("fast_forward_playback_from_frontend")
    .then((response) => console.log(response))
    .catch((error) => console.error(error));
};
