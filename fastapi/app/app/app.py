'''
by: lei.lei.fam.meng@gmail.com
updated: 20241108
'''

import platform
import psutil
from fastapi import FastAPI, File, UploadFile, HTTPException
from fastapi import BackgroundTasks
from fastapi.responses import FileResponse
from fastapi.responses import JSONResponse, StreamingResponse
import os
import socket
import subprocess
import signal
from multiprocessing import Process
import uvicorn
from fastapi.routing import APIRouter
from fastapi import FastAPI, UploadFile, HTTPException, Form,Depends
from pathlib import Path
import zipfile
import shutil
from fastapi import APIRouter, Query, HTTPException
import mmap
import json
import ctypes
from typing import Optional
import importlib
import numpy as np
from fastapi import  WebSocket
from contextlib import asynccontextmanager
from fastapi.middleware.cors import CORSMiddleware
import supervision as sv
import base64
from io import BytesIO
from pydantic import BaseModel
import asyncio
# from cameras.local_image.image_gen import router as image_get_websocket_router
# from cameras.hik.image_gen import router as image_hik_get_websocket_router
# 配置 CORS
import cv2
import io
from ultralytics import YOLO
import torch

origins = [
    "http://localhost:5173",  # 前端地址
    "http://127.0.0.1:5173",  # 或者你的前端地址（如果是 React 本地开发的话）
]

model_path = {
    'luowen_detect':"D:\\code\\tauri-shadcn-template\\fastapi\\app\\algo\\det.pt",
    'diameter_segment':"D:\\code\\tauri-shadcn-template\\fastapi\\app\\algo\\seg.pt",
}

# JSON 数据格式
class ImageMetadata(BaseModel):
    some_field: str  # 示例字段

yolo_model = None
yolo_seg_model = None

    
app = FastAPI()
@app.on_event("startup")
async def startup():
    device = "cuda" if torch.cuda.is_available() else "cpu"
    det_model_str = model_path["luowen_detect"]
    seg_model_str = model_path["diameter_segment"]
    global yolo_model, yolo_seg_model
    yolo_model = YOLO(det_model_str).to(device)
    print("✅ YOLOv8 det模型加载完成")
    yolo_seg_model = YOLO(seg_model_str).to(device)
    await asyncio.sleep(4)  # 等待模型稳定
    print("✅ YOLOv8 seg模型加载完成")
    dummy_img = np.zeros((2448, 2048, 3), dtype=np.uint8)  # 创建黑色图片
    results = yolo_model(dummy_img)[0]
    results = yolo_seg_model(dummy_img)[0]
    torch.cuda.synchronize()
    print("🔥 预热完成，YOLOv8 已准备就绪")



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

@app.get("/warmup")
async def warmup():
    dummy_img = np.zeros((2448, 2048, 3), dtype=np.uint8)  # 创建黑色图片
    results = yolo_model(dummy_img)[0]
    results = yolo_seg_model(dummy_img)[0]
    torch.cuda.synchronize()  # 等待所有 CUDA 操作完成
    return {"msg": "模型已预热"}


def parse_yolo_results(results):
    """手动解析 YOLOv8 目标检测结果为 JSON 格式"""
    if len(results.boxes) == 0:
        return None  # 无检测结果

    boxes = results.boxes.xyxy.cpu().numpy()
    confs = results.boxes.conf.cpu().numpy()
    clss = results.boxes.cls.cpu().numpy()

    max_idx = confs.argmax()

    detection = {
        "x1": float(boxes[max_idx][0]),
        "y1": float(boxes[max_idx][1]),
        "x2": float(boxes[max_idx][2]),
        "y2": float(boxes[max_idx][3]),
        "confidence": float(confs[max_idx]),
        "class_id": int(clss[max_idx]),
    }

    return detection  # 或 json.dumps(detection, indent=4) 如果你需要字符串


@app.post("/detect_diameter")
async def detect_diameter(
    file: UploadFile = File(...),
):
    pass

def get_largest_mask_info(masks):
    largest_contour = None
    max_area = 0
    
    for mask in masks:
        mask = np.array(mask, dtype=np.uint8) * 255
        contours, _ = cv2.findContours(mask, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
        for contour in contours:
            area = cv2.contourArea(contour)
            if area > max_area:
                max_area = area
                largest_contour = contour
    
    if largest_contour is not None:
        (x, y), radius = cv2.minEnclosingCircle(largest_contour)
        return {"center_x": int(x), "center_y": int(y), "diameter": int(radius * 2)}
    
    return None

import time
@app.post("/detect_diameter_with_draw/")
async def detect_diameter_with_draw(
    file: UploadFile = File(...),
):
# 读取图片数据
    print('启动算法')
    t1 =  time.time()
    contents = await file.read()
    image = np.frombuffer(contents, np.uint8)
    image = cv2.imdecode(image, cv2.IMREAD_COLOR)

    # 使用 YOLOv8 进行推理
    try:
        orig_h, orig_w = image.shape[:2]

        results = yolo_seg_model(image)[0]
        detections = sv.Detections.from_ultralytics(results)
        # 绘制mask图片
        mask_annotator = sv.MaskAnnotator()
        label_annotator = sv.LabelAnnotator(text_position=sv.Position.CENTER_OF_MASS)

        annotated_image = mask_annotator.annotate(
            scene=image, detections=detections)
        annotated_image = label_annotator.annotate(
            scene=annotated_image, detections=detections)
        
        detection_json = {}
        for class_name in ["diameter-nei", "diameter-wai"]:
            class_id = next((k for k, v in results.names.items() if v == class_name), None)
            if class_id is None:
                continue
            
            class_id = int(class_id)

            masks = [
                cv2.resize(mask.cpu().numpy(), (orig_w, orig_h), interpolation=cv2.INTER_NEAREST)
                for mask, cls in zip(results.masks.data, results.boxes.cls)
                if int(cls) == class_id
            ]

            if not masks:
                continue

            mask_info = get_largest_mask_info(masks)
            if mask_info:
                detection_json[class_name] = mask_info

        text_position_y = 70  # 从30像素开始绘制文本


        for class_name, info in detection_json.items():
            center_text = f"{class_name}_center: ({info['center_x']}, {info['center_y']})"
            diameter_text = f"{class_name}_diameter: {info['diameter']}"

            font_scale = 2  # 增大字体大小
            font_color = (0, 255, 0)  # 绿色
            thickness = 3  # 增加字体的厚度
            # 绘制文本
            cv2.putText(annotated_image, center_text, 
                        (10, text_position_y), 
                        cv2.FONT_HERSHEY_SIMPLEX, font_scale, font_color, thickness, cv2.LINE_AA)
            text_position_y += 70  # 下一行文本向下偏移50像素

            cv2.putText(annotated_image, diameter_text, 
                        (10, text_position_y), 
                        cv2.FONT_HERSHEY_SIMPLEX, font_scale, font_color, thickness, cv2.LINE_AA)
            text_position_y += 70  # 下一行文本向下偏移50像素


        # **直接用 OpenCV 编码为 PNG**
        _, buffer = cv2.imencode('.jpg', annotated_image)
        img_base64 = base64.b64encode(buffer).decode("utf-8")
        img_bytes = io.BytesIO(buffer)
    except:
        _, buffer = cv2.imencode('.jpg', image)
        img_base64 = base64.b64encode(buffer).decode("utf-8")
        detection_json = {}
        img_bytes = io.BytesIO(buffer)
    print('line 242')
    print(time.time()-t1)
    detection_json['algo_time'] = time.time()-t1
    return {"results": detection_json, "image_base64": img_base64}
    # print(detection_json)
    # 直接返回图片文件
    # headers = {"Content-Disposition": "attachment; filename=detected_image.png"}
    # return StreamingResponse(img_bytes, media_type="image/png", headers=headers, background=JSONResponse(content=detection_json))




@app.post("/detect_luowen_with_draw/")
async def detect_luowen_with_draw(
    file: UploadFile = File(...),
):
    """处理图片，返回检测结果和绘制后的 Base64 图片"""
    print('启动算法')
    t1 =  time.time()
    # 读取图片数据
    contents = await file.read()
    image = np.frombuffer(contents, np.uint8)
    image = cv2.imdecode(image, cv2.IMREAD_COLOR)

    # 使用 YOLOv8 进行推理
    try:

        results = yolo_model(image,conf=0.3)[0]
        detections = sv.Detections.from_ultralytics(results)
        
        # 解析检测结果
        detection_json = parse_yolo_results(results)
        GREEN = (0, 255, 0)
        # 使用 Supervision 绘制检测框
        # image_with_boxes = draw_detections(image, results)
        box_annotator = sv.BoxAnnotator(thickness=5)
        annotated_frame = box_annotator.annotate(
            scene=image.copy(),
            detections=detections)

        # **直接用 OpenCV 编码为 PNG**
        _, buffer = cv2.imencode('.png', annotated_frame)
        img_base64 = base64.b64encode(buffer).decode("utf-8")
        img_bytes = io.BytesIO(buffer)
    except:
        _, buffer = cv2.imencode('.png', image)
        img_base64 = base64.b64encode(buffer).decode("utf-8")
        detection_json = {}
        img_bytes = io.BytesIO(buffer)
    print('line 242')
    print(time.time()-t1)
    return {"results": detection_json, "image_base64": img_base64}

    # 直接返回图片文件
    # headers = {"Content-Disposition": "attachment; filename=detected_image.png"}
    # return StreamingResponse(img_bytes, media_type="image/png", headers=headers)



@app.post("/detect_luowen/")
async def detect_luowen(
    file: UploadFile = File(...),
):
    """处理图片，返回检测结果和绘制后的 Base64 图片"""

    # 读取图片数据
    contents = await file.read()
    image = np.frombuffer(contents, np.uint8)
    image = cv2.imdecode(image, cv2.IMREAD_COLOR)

    # 使用 YOLOv8 进行推理
    try:
        results = yolo_model(image)[0]
        
        # 解析检测结果
        detection_json = parse_yolo_results(results)
    except:
        detection_json = {}

    return {"results": detection_json}



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