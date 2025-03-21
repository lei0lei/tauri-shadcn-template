'''
by: lei.lei.fam.meng@gmail.com
updated: 20241108
'''

import platform
import psutil
from fastapi import FastAPI, File, UploadFile, HTTPException
from fastapi import BackgroundTasks
from fastapi.responses import FileResponse
import os
import socket
import subprocess
import signal
from fastapi.responses import JSONResponse
from multiprocessing import Process
import uvicorn
from fastapi.routing import APIRouter
from fastapi import FastAPI, UploadFile, HTTPException, Form
from pathlib import Path
import zipfile
import shutil
from fastapi import APIRouter, Query, HTTPException
import mmap
import json
import ctypes
from typing import Optional
import importlib

from fastapi import  WebSocket

from fastapi.middleware.cors import CORSMiddleware
# from cameras.local_image.image_gen import router as image_get_websocket_router
# from cameras.hik.image_gen import router as image_hik_get_websocket_router
# 配置 CORS
origins = [
    "http://localhost:5173",  # 前端地址
    "http://127.0.0.1:5173",  # 或者你的前端地址（如果是 React 本地开发的话）
]

app = FastAPI()

app.add_middleware(
        CORSMiddleware,
        allow_origins=["*"],  # can alter with time
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )

# from cameras.local_image.image_gen import get_images_from_folder

# app.include_router(image_get_websocket_router, prefix="/ws/local", tags=["WebSocket","local",'stream'])
# app.include_router(image_hik_get_websocket_router, prefix="/ws/hik", tags=["WebSocket","hik",'stream'])
# @app.websocket("/ws")
# async def websocket_endpoint(websocket: WebSocket):
#     await websocket.accept()
#     async for img_bytes in get_images_from_folder():
#         await websocket.send_bytes(img_bytes)  # 向前端发送二进制数据

@app.get("/")
async def root():
    '''
    显示算法列表和命令列表
    '''
    return {"Algo list": "Hello World",
            "Command": "run"}
    
@app.get("/data-transfer-protocol")
async def data_transfer_protocol():
    '''
    显示数据传输示例
    '''
    return {"machine": "Hello World",
            "app": "run"}
    
    
@app.get("/show-algo-list")
async def show_algo_list():
    '''
    列出算法信息
    '''
    return {"machine": "Hello World",
            "app": "run"}
############################### 算法文件上传及删除 ###############################
@app.post("/upload-algo")
async def upload_algo(zip_file: UploadFile, target_dir: str = Form(...)):
    '''
    上传算法zip, 默认存放在algorithms目录下并且以该目录为相对路径
    '''
    # 确保目标目录存在
    target_dir = os.path.join('./detect_server/algorithms',target_dir)
    target_path = Path(target_dir)
    if not target_path.exists():
        # 创建文件夹
        os.makedirs(target_dir, exist_ok=True)
    else:
        raise HTTPException(status_code=400, detail="target algorithms exit,delete old ones or upload to new position.")
    # 获取上传的文件内容
    file_location = f"./{zip_file.filename}"
    with open(file_location, "wb") as file_object:
        file_object.write(await zip_file.read())

    # 解压文件到指定目录
    try:
        with zipfile.ZipFile(file_location, 'r') as zip_ref:
            zip_ref.extractall(target_path)
        os.remove(file_location)  # 删除临时存储的zip文件
    except zipfile.BadZipFile:
        raise HTTPException(status_code=400, detail="Uploaded file is not a valid zip file.")

    return {"message": f"File successfully uploaded and extracted to {target_dir}"}
    
@app.post("/delete-algo")
async def delete_algo(target_dir: str = Form(...)):
    '''
    删除目标算法文件夹及其内容
    '''
    # 构建完整的目标目录路径
    target_dir = os.path.join('./detect_server/algorithms', target_dir)
    target_path = Path(target_dir)

    # 检查目录是否存在
    if not target_path.exists():
        raise HTTPException(status_code=404, detail="Directory not found")

    # 删除目标目录及其内容
    try:
        shutil.rmtree(target_path)  # 递归删除文件夹及其内容
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error occurred while deleting the directory: {str(e)}")

    return {"message": f"Directory '{target_dir}' has been successfully deleted."}
    
############################### 动态添加算法路由 ###############################
# 存放路由路径和推理闭包
router_dict = {}
inference_cache = {}

# 动态创建和注册算法路由
def register_algorithm_route(algo_name: str, algo_path: str):
    """
    动态注册新的路由
    """
    if algo_path in router_dict:
        raise HTTPException(status_code=400, detail=f"Algorithm '{algo_path}' is already registered, please delete old ones first.")

    # 创建一个新的子路由
    new_router = APIRouter()

    @new_router.get(f"/{algo_name}")
    async def algo_inference():
        """
        在此处执行算法的推理任务
        """
        # 动态导入推理函数
        algo_path = './detect_server/algorithms/'+algo_name+'/startup'
        
        module_path = algo_path.replace("/", ".").lstrip(".")  # 去掉前面的 "./"
        inference, release_inference = getattr(__import__(module_path, fromlist=["start"]), "start")()
        inference_cache[algo_name] = (inference, release_inference)
        # 启动算法，假设算法路径指向可执行的算法
        inference_function, _ = inference_cache[algo_name]
        inference_function = inference_function(None,None)
        
        return {"message": f"Running algorithm: {algo_name}"}

    # 注册路由到 FastAPI
    app.include_router(new_router)
    router_dict[algo_name] = new_router
    
    return {"message": f"Algorithm '{algo_name}' registered successfully."}


@app.post("/register-algo")
async def register_algo(algo_name: str, algo_path: str):
    """
    注册一个新的算法路由，传入算法名称和路径。
    """
    # 校验路径是否存在
    algo_path = os.path.join('./detect_server/algorithms',algo_path)
    if not Path(algo_path).exists():
        raise HTTPException(status_code=400, detail=f"Algorithm path '{algo_path}' not found, please upload first.")
    
    # 校验算法格式是否正确
    
    # 动态注册算法路由
    return register_algorithm_route(algo_name, algo_path)

@app.post("/unregister-algo")
async def unregister_algo(algo_name: str, algo_path: str):
    """
    取消注册一个算法路由，并且释放资源。
    """
    # 校验路径是否存在
    algo_path = os.path.join('./detect_server/algorithms',algo_path)
    if not Path(algo_path).exists():
        raise HTTPException(status_code=400, detail=f"Algorithm path '{algo_path}' not found, please upload first.")
    print(inference_cache)
    if algo_name not in inference_cache:
        return {"error": f"Algorithm '{algo_name}' not found or not registered."}
    # 释放资源并清理缓存
    _, release_resources = inference_cache[algo_name]
    release_resources()  # 调用释放资源的函数
    del inference_cache[algo_name]  # 删除缓存
    del router_dict[algo_name]  # 删除路由
    # 动态注册算法路由
    return {"unregistered_algorithm": algo_name}


@app.get("/registered-algos")
async def list_registered_algos():
    """
    查看已注册的算法列表
    """
    return {"registered_algorithms": list(router_dict.keys())}

# @app.websocket("/ws")
# async def websocket_endpoint(websocket: WebSocket):
#     await websocket.accept()
#     while True:
#         data = await websocket.receive_text()
#         await websocket.send_text(f"Message text was: {data}")




# 推理算法 返回结果




# 推理算法 返回结果加绘图