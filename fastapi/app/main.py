'''
by: lei.lei.fam.meng@gmail.com
updated: 20240121
'''

import platform
import psutil
from fastapi import FastAPI, File, UploadFile, HTTPException
from fastapi import BackgroundTasks
from fastapi.responses import FileResponse
import os

from fastapi.responses import JSONResponse
from multiprocessing import Process

from fastapi.routing import APIRouter
from fastapi import FastAPI, UploadFile, HTTPException, Form
from pathlib import Path
import zipfile
import shutil
from fastapi import APIRouter, Query, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from typing import Optional
from fastapi import  WebSocket