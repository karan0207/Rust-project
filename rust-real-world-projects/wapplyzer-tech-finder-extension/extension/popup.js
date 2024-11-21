document.addEventListener('DOMContentLoaded', async () => {
    const techList = document.getElementById('tech-list');
    const detectedTechs = await detectTechnologies(); // You would need to call the function here or use messaging to fetch results
    detectedTechs.forEach(tech => {
      const listItem = document.createElement('li');
      listItem.textContent = tech;
      techList.appendChild(listItem);
    });
  });
  