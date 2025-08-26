import DefaultTheme from "vitepress/theme";
import { onMounted } from "vue";
import "./custom.css";

export default {
  ...DefaultTheme,
  setup() {
    onMounted(() => {
      const target = document.querySelector("img#mago-logo");
      if (!target) {
        console.warn("3D hover effect: target element not found");
        return;
      }

      const intensity = 20;

      target.addEventListener("mouseenter", () => {
        target.classList.add("is-interactive");
      });

      target.addEventListener("mousemove", (e) => {
        const { left, top, width, height } = target.getBoundingClientRect();
        const x = (e.clientX - left - width / 2) / (width / 2);
        const y = (e.clientY - top - height / 2) / (height / 2);

        target.style.transform = `
          perspective(1000px)
          rotateY(${x * intensity}deg)
          rotateX(${-y * intensity}deg)
          scale3d(1, 1, 1)
        `;
      });

      target.addEventListener("mouseleave", () => {
        target.style.transform = "perspective(1000px) rotateY(0) rotateX(0) scale3d(1, 1, 1)";
        target.classList.remove("is-interactive");
      });
    });
  },
};
