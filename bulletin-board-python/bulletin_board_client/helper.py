from .bulletin_board_client import *
import numpy as np
import os

def post(title, second, third=None):
    '''Posts an ArrayObject.'''
    match third:
        case None:
            val=second
            tag='Python'
        case _:
            val=third
            tag=second
    match val:
        case int():
            post_integer(title, tag, val)
        case float():
            post_real(title, tag, val)
        case complex():
            post_complex(title, tag, val)
        case str():
            post_string(title, tag, val)
        case list():
            post(title, tag, np.array(val))
        case np.ndarray():
            if val.size == 0 :
                raise Exception("Array size cannot be zero")
            shape = list(val.shape)
            data = val.flatten(order='C')
            match data[0]:
                case np.int64():
                    post_integer_array(title, tag, data, shape)
                case np.float64():
                    post_real_array(title, tag, data, shape)
                case np.complex128():
                    post_complex_array(title, tag, data, shape)
                case np.str_():
                    post_string_array(title, tag, data, shape)
                case _:
                    raise Exception("Wrong type")
        case _:
            raise Exception("Wrong type")

def to_array(data):
    match data:
        case tuple():
            arr = np.array(data[0])
            arr.reshape(data[1], order='C')
            return arr
        case _:
            return data

def read(title, tag=None, revisions=None):
    '''Reads ArrayObjects.'''
    match revisions:
        case int():
            revisions = [revisions]
    data = read_raw(title, tag, revisions)
    converted = list(map(to_array, data))
    if len(converted) == 1:
        return converted[0]
    else:
        return converted

def status():
    '''Returns the status of the server.'''
    data = status_raw()
    return {
        "datasize": data[0],
        "memory_used": data[1],
        "memory_used(%)": data[2],
        "objects": data[3],
        "files": data[4],
        "archived": data[5]
    }

def board_listing(data):
    return {
        "title": data[0],
        "tag": data[1],
        "revisions": data[2]
    }

def view_board():
    '''Returns the list of the bulletins.'''
    data = view_board_raw()
    return list(map(board_listing, data))

def bulletin_listing(data):
    return {
        "revision": data[0],
        "datasize": data[1],
        "timestamp": data[2],
        "backend": data[3]
    }

def get_info(title, tag=None):
    '''Returns the details of a bulletin.'''
    data = get_info_raw(title, tag)
    return list(map(bulletin_listing, data))

def clear_revisions(title, second, third=None):
    '''Deletes specific revisions from a bulletin.'''
    match third:
        case int():
            clear_revisions_raw(title, [third], second)
        case list():
            clear_revisions_raw(title, third, second)
        case None:
            match second:
                case int():
                    clear_revisions_raw(title, [second])
                case list():
                    clear_revisions_raw(title, second)
