import json
import asyncio
from fastapi import FastAPI
from sse_starlette.sse import EventSourceResponse
import uvicorn
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI()
app.add_middleware(    
  CORSMiddleware,    
  allow_origins=['*'], # 设置允许跨域的域名列表，* 代表所有域名    
  allow_credentials=True,    
  allow_methods=['*'],    
  allow_headers=['*'],
)

async def event_generator():    
  count = 0    
  while True:        
    await asyncio.sleep(1)        
    count += 1        
    data = {"count": count}        
    yield json.dumps(data)
    
@app.get("/events")
async def get_events():    
  return EventSourceResponse(event_generator())
  
@app.post("/events")
async def post_events():      
  return EventSourceResponse(event_generator())
  
if __name__ == '__main__':    
  uvicorn.run(app, host='0.0.0.0', port=8000)
  
