import { writeStorage } from "./shared/storage";

(() => {
  const token = document.body.dataset.token || "";
  const target = document.body.dataset.target || "/";
  if (token) {
    writeStorage("lgxpkf.session", token);
  }
  if (target.startsWith("/") && !target.startsWith("//") && !target.includes("://")) {
    window.location.replace(target);
  } else {
    window.location.replace("/");
  }
})();
