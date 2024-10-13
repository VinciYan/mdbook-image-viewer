document.addEventListener('DOMContentLoaded', function() {
    // 查找所有的 <ol> 元素
    const listItems = document.querySelectorAll('ol');
    
    listItems.forEach(ol => {
      // 检查 <ol> 后面是否紧跟着 <p/> 元素
      let nextElement = ol.nextElementSibling;
      // if (nextElement && nextElement.tagName === 'P') {
      //   // 将 <p> 元素移动到 <ol> 内部
      //   ol.appendChild(nextElement);
      //   console.log("移动成功")
      // }


      if (nextElement && nextElement.tagName === 'DIV') {
        let nextNextElement = nextElement.nextElementSibling;
        if (nextNextElement && nextNextElement.tagName === 'P') {
            // 将 <p> 元素移动到 <ol> 内部            
            ol.appendChild(nextElement);
            ol.appendChild(nextNextElement)
            console.log("移动成功")
          }
      }
      
    });
  });