<template>
    <div class="floating-shapes">
      <div class="shape shape-1"></div>
      <div class="shape shape-2"></div>
      <div class="shape shape-3"></div>
      <div class="shape shape-4"></div>
    </div>
</template>

<script setup lang="ts">
</script>

<style lang="css" scoped>
.floating-shapes {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
  z-index: 1;
}

.shape {
  position: absolute;
  border-radius: 50%;
  animation: float 4s ease-in-out infinite;
}

.shape-1 {
  width: 120px;
  height: 120px;
  top: 10%;
  left: 10%;
  animation-delay: 0s;
  background: linear-gradient(45deg, rgba(59, 130, 246, 0.2), rgba(147, 51, 234, 0.15));
}

.shape-2 {
  width: 180px;
  height: 180px;
  top: 60%;
  right: 5%;
  animation-delay: 1.5s;
  background: linear-gradient(45deg, rgba(16, 185, 129, 0.2), rgba(6, 182, 212, 0.15));
}

.shape-3 {
  width: 100px;
  height: 100px;
  top: 15%;
  right: 15%;
  animation-delay: 3s;
  background: linear-gradient(45deg, rgba(245, 101, 101, 0.2), rgba(251, 146, 60, 0.15));
}

.shape-4 {
  width: 150px;
  height: 150px;
  bottom: 15%;
  left: 15%;
  animation-delay: 0.8s;
  background: linear-gradient(45deg, rgba(139, 92, 246, 0.2), rgba(236, 72, 153, 0.15));
}

@keyframes float {
  0%, 100% { 
    transform: translateY(0px) rotate(0deg) scale(1); 
  }
  25% { 
    transform: translateY(-40px) rotate(15deg) scale(1.1); 
  }
  50% { 
    transform: translateY(-25px) rotate(-10deg) scale(0.95); 
  }
  75% { 
    transform: translateY(-35px) rotate(20deg) scale(1.05); 
  }
}
</style>