var setVh=(()=>{let h=`0`,g=`px`,f=0.01;var a=document.documentElement.clientHeight*f;document.documentElement.style.setProperty(`--1svh`,a+ g);var b=window.innerHeight*f;document.documentElement.style.setProperty(`--1dvh`,b+ g);if(document.body){var c=document.createElement(`div`);c.style.width=`1px`;c.style.height=`100vh`;c.style.position=`fixed`;c.style.left=h;c.style.top=h;c.style.bottom=h;c.style.visibility=`hidden`;document.body.appendChild(c);var d=c.clientHeight;c.remove();var e=d*f;document.documentElement.style.setProperty(`--1lvh`,e+ g)}});var isMobile=(()=>{if(/Android|iPhone|iPad|iPod/i.test(navigator.userAgent)||navigator.userAgent.match(/Mac/)&&navigator.maxTouchPoints&&navigator.maxTouchPoints>2){return !0};return !1});var initialize=(()=>{if(typeof window===`undefined`){return};if(`CSS` in window&&`supports` in window.CSS&&window.CSS.supports(`height: 100svh`)&&window.CSS.supports(`height: 100dvh`)&&window.CSS.supports(`height: 100lvh`)){return};if(!isMobile){return};setVh();document.addEventListener(`DOMContentLoaded`,(()=>{setVh()}));window.addEventListener(`resize`,(()=>{setVh()}))});initialize()